use crate::ngrams::NgramData;

use itertools::Itertools;

pub struct ProgrammingData();

// Source: https://github.com/e3b0c442/keywords/blob/main/chart.yaml
/*
    Python processing of that document after removing duplicate languages. Kept the newest versions.

    from collections import Counter
    import yaml

    data = open(/path/to/yaml).read()
    parsed = yaml.unsafe_load(data)
    keywords = Counter()
    for item in parsed:
        keywords.update([str(word).lower() for word in item["keywords"]])
    for word in sorted(word for word, _ in keywords.most_common(100)):
        print(word)
*/
#[rustfmt::skip]
const PROGRAMMING_KEYWORDS: [&str; 100] = [
    "abstract", "and", "as", "assert", "async", "auto", "await", "bool", "break", "byte",
    "case", "catch", "char", "class", "const", "continue", "def", "default", "do", "double",
    "else", "elseif", "end", "enum", "explicit", "extends", "extern", "false", "final", "finally",
    "float", "for", "from", "function", "get", "global", "goto", "if", "implements", "import",
    "in", "inline", "instanceof", "int", "interface", "is", "let", "long", "match", "module",
    "namespace", "new", "nil", "not", "null", "object", "open", "operator", "or", "out",
    "override", "package", "private", "protected", "public", "register", "repeat", "return", "sealed", "select",
    "self", "set", "short", "signed", "sizeof", "static", "struct", "super", "switch", "then",
    "this", "throw", "true", "try", "type", "typedef", "typeof", "union", "unsigned", "until",
    "using", "var", "void", "volatile", "when", "where", "while", "with", "xor", "yield",
];

fn char_windows(source: &str, size: usize) -> impl Iterator<Item = &str> {
    source.char_indices().flat_map(move |(from, _)| {
        source[from..]
            .char_indices()
            .nth(size - 1)
            .map(|(to, c)| &source[from..from + to + c.len_utf8()])
    })
}

impl NgramData for ProgrammingData {
    /// Returns a Vec<String> of the most common bigrams for Programmers.
    ///
    /// The collection is the bigrams from the most common keywords across programming languages.
    fn get_bigrams(&self) -> Vec<String> {
        PROGRAMMING_KEYWORDS
            .iter()
            .flat_map(|word| char_windows(word, 2))
            .unique()
            .map(String::from)
            .collect()
    }

    /// Returns a Vec<String> of the most common trigrams for Programmers.
    fn get_trigrams(&self) -> Vec<String> {
        PROGRAMMING_KEYWORDS
            .iter()
            .filter(|w| w.len() >= 3)
            .flat_map(|word| char_windows(word, 3))
            .unique()
            .map(String::from)
            .collect()
    }

    /// Returns a Vec<String> of the most common tetragrams for Programmers.
    fn get_tetragrams(&self) -> Vec<String> {
        PROGRAMMING_KEYWORDS
            .iter()
            .filter(|w| w.len() >= 4)
            .flat_map(|word| char_windows(word, 4))
            .unique()
            .map(String::from)
            .collect()
    }

    /// Returns a Vec<String> of the most common words for Programmers.
    fn get_wordlist(&self) -> Vec<String> {
        PROGRAMMING_KEYWORDS.into_iter().map(String::from).collect()
    }
}
