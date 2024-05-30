pub mod english;
pub mod programming;

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum NgramGroup {
    #[default]
    English,
    Programming,
}

pub trait NgramData {
    fn get_bigrams(&self) -> Vec<String>;
    fn get_trigrams(&self) -> Vec<String>;
    fn get_tetragrams(&self) -> Vec<String>;
    fn get_wordlist(&self) -> Vec<String>;
}

// Returns a Vec<String> of ngrams from a comma separated file.
// The file must include no spaces between the commas. Line breaks are allowed.
//
// # Example file content:
// ```
// the,and,ing,ion,tio,ent,
// all,ons
// ```
pub fn get_from_file<'a>(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;
    let ngrams: Vec<String> = file
        .lines()
        .map(|line| line.split(","))
        .flatten()
        .map(|s| s.to_string())
        .collect();
    Ok(ngrams)
}
