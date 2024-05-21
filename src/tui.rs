use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect}, prelude::{CrosstermBackend, Stylize, Terminal}, symbols::border, terminal::Frame, text::{Line, Span}, widgets::{block::Title, Block, Borders, Paragraph}
};

use std::io::{self, stdout, Stdout};

use crate::AppState;
use crate::Args;

fn render(frame: &mut Frame, state: &AppState, args: &Args, kb_string: &String, cat_string: &String) {
    let area = match args.nokb {
        true => Rect { x: 0, y: 0, width: 79, height: 13 },
        false => Rect { x: 0, y: 0, width: 79, height: 19 },
    };
    let inner = area.inner(&Margin::new(1, 1));
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Min(2), // Title header
            Constraint::Min(2), // lesson number + successes and fails
            Constraint::Percentage(100), // lesson content
            Constraint::Min(2), // wpm and accuracy stats and threshold
        ])
        .split(inner);


    let help: Title = Title::from(Line::from(vec![
        " Quit ".into(),
        "<esc> ".blue().bold(),
    ]));

    let outline: Block = Block::default()
        .title(help.alignment(Alignment::Right))
        .borders(Borders::ALL)
        .border_set(border::ROUNDED);

    frame.render_widget(
        outline,
        area,
    );

    let title_block: Block = Block::default()
        .borders(Borders::BOTTOM)
        .border_set(border::ROUNDED);
    frame.render_widget(
        title_block,
        layout[0],
    );
    let title: Paragraph = Paragraph::new(Line::from(vec![
        "               ngrrram!".bold(),
        "   by winterveil".italic().gray(),
    ]));
    frame.render_widget(
        title.alignment(Alignment::Center),
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(100),
            ])
            .split(layout[0])[0],
    );

    let lesson_stats_block: Block = Block::default()
        .borders(Borders::BOTTOM)
        .border_set(border::ROUNDED);
    frame.render_widget(
        lesson_stats_block,
        layout[1],
    );
    let current_lesson_number = state.current_lesson_number;
    let lesson_number: Paragraph = Paragraph::new(
        format!("    Lesson #{current_lesson_number}")
    ).alignment(Alignment::Left);

    let successes = state.succeeded_lessons;
    let fails = state.failed_lessons;
    let lesson_stats: Paragraph = Paragraph::new(
        Line::from(vec![
            format!("✔: {}, ", successes).green(),
            format!("✘: {}    ", fails).red(),
        ])
    ).alignment(Alignment::Right);
    frame.render_widget(
        lesson_number,
        layout[1],
    );
    frame.render_widget(
        lesson_stats,
        layout[1],
    );

    let lesson_block: Block = Block::default()
        .borders(Borders::BOTTOM)
        .border_set(border::ROUNDED);
    frame.render_widget(
        lesson_block,
        layout[2],
    );

    // we have 9 lines of inner content here
    let mut correction_line: String = "".to_string();
    let mut lesson_letters: Vec<Span> = Vec::new();
    for (idx, c) in state.current_lesson_string.chars().enumerate() {
        let typed = state.current_typed_string.chars().nth(idx);
        let (span, correction) = if typed == Some(c) {
            (Span::from(c.to_string()).green().bold(), ' ')
        } else if typed.is_some() {
            if c == ' ' { // we display mistyped spaces as dots so they are more visible
                (Span::from("•").red().bold(), typed.unwrap())
            } else {
                (Span::from(c.to_string()).red().bold(), typed.unwrap())
            }
        } else {
            (Span::from(c.to_string()).bold(), ' ')
        };
        correction_line.push(correction);
        lesson_letters.push(span);
    }

    let lesson_line: Line = Line::from(lesson_letters);

    let lesson: Paragraph = Paragraph::new(vec![
        "\n\n".into(),
        lesson_line,
        correction_line.italic().gray().into(),
    ]).alignment(Alignment::Center);
    frame.render_widget(
        lesson,
        layout[2],
    );
    if !args.nokb {
        // TODO: properly indent kb_string
        let keyboard_display: Paragraph = Paragraph::new(kb_string.to_owned()).gray();
        let mut indent = 5;
        if !args.cat {
            indent = 13;
        }

        frame.render_widget(
            keyboard_display,
            Rect::new(layout[2].x + indent, layout[2].y + 3, layout[2].width - indent, layout[2].height - 4),
        );
    }

    // draw cursor
    // get the the offset of the lesson_line since it it centered
    let lesson_line_offset = layout[2].width as usize / 2 - state.current_lesson_string.len() / 2;
    let cursor_x = state.current_typed_string.len() + lesson_line_offset;
    let cursor_y = 1;
    frame.set_cursor(
        layout[2].x + cursor_x as u16,
        layout[2].y + cursor_y as u16,
    );

    // WPM and ACCURACY stats
    let last_wpm = state.wpm_history.last().cloned().unwrap_or(0);
    let last_acc = state.acc_history.last().cloned().unwrap_or(0);
    let wpm_width = 3;
    let acc_width = 3;
    let wpm_line: Line = Line::from(vec![
        Span::from(format!("  need >= {:width$}WPM,   avg. ", state.need_wpm, width = wpm_width)).gray().into(),
        Span::from(format!("{:width$}WPM", state.average_wpm, width = wpm_width)).into(),
        Span::from(",   last" ).gray().into(),
        Span::from(format!(" {:width$}WPM", last_wpm, width = wpm_width)).into(),
    ]);
    let acc_line: Line = Line::from(vec![
        Span::from(format!("  need >= {:width$}% Acc, avg. ", state.need_acc, width = acc_width)).gray().into(),
        Span::from(format!("{:width$}% Acc", state.average_accuracy, width = acc_width)).into(),
        Span::from(", last" ).gray().into(),
        Span::from(format!(" {:width$}% Acc", last_acc, width = acc_width)).into(),
    ]);

    let stats: Paragraph = Paragraph::new(vec![
        wpm_line,
        acc_line,
    ]);
    frame.render_widget(
        stats,
        layout[3],
    );

    if args.cat {
        let cat: Paragraph = Paragraph::new(cat_string.to_string()).reset();
        frame.render_widget(
            cat,
            Rect::new(layout[2].x + 60, layout[2].y + 5, layout[2].width - 60, layout[2].height-2),
        );
    }


}

pub fn init_tui() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    // setup terminal to restore later
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

pub fn cleanup_tui() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn ensure_screen_size(terminal: &mut Terminal<CrosstermBackend<Stdout>>, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let size = terminal.size()?;
    if args.nokb {
        if size.width < 79 || size.height < 13 {
            terminal.draw(|frame: &mut Frame| {
                let warning = Paragraph::new("Please resize your terminal to at least 80x13.\n<esc> or <ctrl-c> to quit").alignment(Alignment::Center);
                frame.render_widget(
                    warning,
                    Rect { x: 0, y: 0, width: size.width, height: size.height },
                );
            })?;
            return Err(io::Error::new(io::ErrorKind::Other, "Terminal too small").into());
        }
    } else {
        // we need more space to display the keyboard
        if size.width < 79 || size.height < 20 {
            terminal.draw(|frame: &mut Frame| {
                let warning = Paragraph::new("Please resize your terminal to at least 80x20,\nor consider disabling the keyboard display with --nokb\n\n<esc> or <ctrl-c> to quit").alignment(Alignment::Center);
                frame.render_widget(
                    warning,
                    Rect { x: 0, y: 0, width: size.width, height: size.height },
                );
            })?;
            return Err(io::Error::new(io::ErrorKind::Other, "Terminal too small").into());
        }
    }
    Ok(())
}

pub fn render_tui(state: &AppState, terminal: &mut Terminal<CrosstermBackend<Stdout>>, args: &Args, kb_string: &String, cat_string: &String) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|frame: &mut Frame| {
        render(frame, state, args, kb_string, cat_string);
    })?;

    Ok(())
}
