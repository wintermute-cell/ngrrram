use clap::Parser;

mod cat;
mod game;
mod layout;
mod ngrams;
mod tui;

use ngrams::{english::EnglishData, programming::ProgrammingData, NgramData, NgramGroup};

#[derive(Parser)]
//#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'g',
        long,
        value_enum,
        default_value_t = NgramGroup::default(),
        help = "which ngram group to work from. Defaults is {NgramGroup::default:?}"
    )]
    ngram_group: NgramGroup,

    #[arg(
        short,
        long,
        default_value = "2",
        value_name = "2|3|4|w|file",
        help = "use bi-(2), tri-(3), tetragrams(4), (w)ords or comma separated wordlist file."
    )]
    n: String,

    #[arg(
        short,
        long,
        default_value = "50",
        value_name = "1-200",
        help = "use the top X ngrams ordered by usage."
    )]
    top: i32,

    #[arg(
        short,
        long,
        default_value = "2",
        value_name = "1-200",
        help = "how many different ngrams to use in a single lesson."
    )]
    combi: i32,

    #[arg(
        short,
        long,
        default_value = "3",
        value_name = "number",
        help = "how often to repeat *each* different ngram in a lesson."
    )]
    rep: i32,

    #[arg(
        short,
        long,
        default_value = "40",
        value_name = "number",
        help = "the wpm threshold at which the lesson is considered a success."
    )]
    wpm: i32,

    #[arg(
        short,
        long,
        default_value = "94",
        value_name = "0-100",
        help = "the accuracy in percent at which the lesson is considered a success."
    )]
    acc: i32,

    #[arg(
        long,
        action,
        default_value = "",
        value_name = "layout",
        help = "your current keyboard layout. only needed if you want to emulate a different layout. see docs for supported layouts."
    )]
    emu_in: String,

    #[arg(
        long,
        action,
        default_value = "",
        value_name = "layout",
        help = "the layout you want to emulate. only needed if you want to emulate a different layout. see docs for supported layouts."
    )]
    emu_out: String,

    #[arg(
        long,
        action,
        help = "pass this flag to disable the keyboard layout display."
    )]
    nokb: bool,

    #[arg(long, action, help = "the most important flag. don't practice alone.")]
    cat: bool,
}

struct AppState {
    current_lesson_number: i32,
    succeeded_lessons: i32,
    failed_lessons: i32,
    wpm_history: Vec<i32>,
    average_wpm: i32,
    acc_history: Vec<i32>,
    average_accuracy: i32,
    current_lesson_string: String,
    current_typed_string: String,
    ngrams: Vec<String>,

    // wpm and acc tracking
    need_wpm: i32,
    need_acc: i32,
    acc_key_hits: i32,
    acc_key_misses: i32,
    wpm_start_time: std::time::Instant,

    // emulation
    use_emulation: bool,
}

fn get_ngrams(args: &Args) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ngram_data: Box<dyn NgramData> = match args.ngram_group {
        NgramGroup::English => Box::new(EnglishData {}),
        NgramGroup::Programming => Box::new(ProgrammingData {}),
    };

    match args.n.as_str() {
        "2" => Ok(ngram_data.get_bigrams()),
        "3" => Ok(ngram_data.get_trigrams()),
        "4" => Ok(ngram_data.get_tetragrams()),
        "w" => Ok(ngram_data.get_wordlist()),
        &_ => ngrams::get_from_file(&args.n),
    }
}

fn validate_args(args: &Args) -> bool {
    if args.top < 1 || args.top > 200 {
        println!("Invalid argument for top. Use a number between 1 and 200.");
        return false;
    }
    if args.combi < 1 || args.combi > 200 {
        println!("Invalid argument for combi. Use a number between 1 and 200.");
        return false;
    }
    if args.rep < 1 || args.rep > 200 {
        println!("Invalid argument for rep. Use a number between 1 and 200.");
        return false;
    }
    if args.wpm < 1 || args.wpm > 200 {
        println!("Invalid argument for wpm. Use a number between 1 and 200.");
        return false;
    }
    if args.acc < 0 || args.acc > 100 {
        println!("Invalid argument for acc. Use a number between 0 and 100.");
        return false;
    }
    if args.emu_in != "" && args.emu_out == "" {
        println!("You need to specify both emu_in and emu_out.");
        return false;
    }
    if args.emu_in == "" && args.emu_out != "" {
        println!("You need to specify both emu_in and emu_out.");
        return false;
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !validate_args(&args) {
        std::process::exit(1);
    }

    // TODO: refactor this into a function
    let in_layout = match args.emu_in.as_str() {
        "qwerty" => layout::Layout::Qwerty,
        "qwertz" => layout::Layout::Qwertz,
        "azerty" => layout::Layout::Azerty,
        "dvorak" => layout::Layout::Dvorak,
        "colemak" => layout::Layout::Colemak,
        "colemakdh" => layout::Layout::ColemakDH,
        &_ => layout::Layout::Qwerty,
    };
    let out_layout = match args.emu_out.as_str() {
        "qwerty" => layout::Layout::Qwerty,
        "qwertz" => layout::Layout::Qwertz,
        "azerty" => layout::Layout::Azerty,
        "dvorak" => layout::Layout::Dvorak,
        "colemak" => layout::Layout::Colemak,
        "colemakdh" => layout::Layout::ColemakDH,
        &_ => layout::Layout::Qwerty,
    };
    let out_layout_string = layout::get_layout_string(&out_layout);

    let mut state = AppState {
        current_lesson_number: 0,
        succeeded_lessons: 0,
        failed_lessons: 0,
        wpm_history: Vec::new(),
        average_wpm: 0,
        acc_history: Vec::new(),
        average_accuracy: 0,
        current_lesson_string: "".to_string(),
        current_typed_string: "".to_string(),
        ngrams: vec![],

        need_wpm: args.wpm,
        need_acc: args.acc,

        acc_key_hits: 0,
        acc_key_misses: 0,
        wpm_start_time: std::time::Instant::now(),

        use_emulation: args.emu_in != "" && args.emu_out != "", // only use emulation if both are set
    };

    state.ngrams = get_ngrams(&args)?;

    let mut terminal = tui::init_tui()?;

    let mut kb_emu = layout::KbEmulator::new(in_layout, out_layout);

    let mut cat_iter = cat::cat();
    let mut cat_frame: String = cat_iter.next().expect("cat frame not found").to_string();

    let mut dt: std::time::Duration;
    let mut cat_timer = std::time::Duration::from_millis(0);

    loop {
        let now = std::time::Instant::now();
        if let Ok(_) = tui::ensure_screen_size(&mut terminal, &args) {
            tui::render_tui(&state, &mut terminal, &args, &out_layout_string, &cat_frame)?;
        }
        let should_quit = game::run_game(&args, &mut state, &mut kb_emu)?;
        if should_quit {
            break;
        }
        dt = now.elapsed();
        cat_timer += dt;
        if cat_timer.as_millis() > 160 {
            cat_timer = std::time::Duration::from_millis(0);
            cat_frame = cat_iter.next().expect("cat frame not found").to_string();
        }
    }

    tui::cleanup_tui()?;

    Ok(())
}
