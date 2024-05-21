

// ngrams from
// - https://github.com/KaranAgarwalla/Encryption-Decryption/blob/master/ngrams2.txt
// - https://github.com/KaranAgarwalla/Encryption-Decryption/blob/master/ngrams3.txt
// - https://github.com/KaranAgarwalla/Encryption-Decryption/blob/master/ngrams4.txt


// Returns a Vec<String> of the most common bigrams in the English language.
pub fn get_bigrams() -> Vec<String> {
    let bigrams = vec![
        "th", "he", "in", "er", "an", "re", "on", "at", "en", "nd", "ti", "es", "or", "te", "of", "ed",
        "is", "it", "al", "ar", "st", "to", "nt", "ng", "se", "ha", "as", "ou", "io", "le", "ve", "co",
        "me", "de", "hi", "ri", "ro", "ic", "ne", "ea", "ra", "ce", "li", "ch", "ll", "be", "ma", "si",
        "om", "ur", "ca", "el", "ta", "la", "ns", "di", "fo", "ho", "pe", "ec", "pr", "no", "ct", "us",
        "ac", "ot", "il", "tr", "ly", "nc", "et", "ut", "ss", "so", "rs", "un", "lo", "wa", "ge", "ie",
        "wh", "ee", "wi", "em", "ad", "ol", "rt", "po", "we", "na", "ul", "ni", "ts", "mo", "ow", "pa",
        "im", "mi", "ai", "sh", "ir", "su", "id", "os", "iv", "ia", "am", "fi", "ci", "vi", "pl", "ig",
        "tu", "ev", "ld", "ry", "mp", "fe", "bl", "ab", "gh", "ty", "op", "wo", "sa", "ay", "ex", "ke",
        "fr", "oo", "av", "ag", "if", "ap", "gr", "od", "bo", "sp", "rd", "do", "uc", "bu", "ei", "ov",
        "by", "rm", "ep", "tt", "oc", "fa", "ef", "cu", "rn", "sc", "gi", "da", "yo", "cr", "cl", "du",
        "ga", "qu", "ue", "ff", "ba", "ey", "ls", "va", "um", "pp", "ua", "up", "lu", "go", "ht", "ru",
        "ug", "ds", "lt", "pi", "rc", "rr", "eg", "au", "ck", "ew", "mu", "br", "bi", "pt", "ak", "pu",
        "ui", "rg", "ib", "tl", "ny", "ki", "rk", "ys", 
    ];
    bigrams.iter().map(|s| s.to_string()).collect()
}

// Returns a Vec<String> of the most common trigrams in the English language.
pub fn get_trigrams() -> Vec<String> {
    let trigrams = vec![
      "the", "and", "ing", "ion", "tio", "ent", "ati", "for", "her", "ter", "hat", "tha", "ere", "ate",
        "his", "con", "res", "ver", "all", "ons", "nce", "men", "ith", "ted", "ers", "pro", "thi",
        "wit", "are", "ess", "not", "ive", "was", "ect", "rea", "com", "eve", "per", "int", "est",
        "sta", "cti", "ica", "ist", "ear", "ain", "one", "our", "iti", "rat", "nte", "tin", "ine",
        "der", "ome", "man", "pre", "rom", "tra", "whi", "ave", "str", "act", "ill", "ure", "ide",
        "ove", "cal", "ble", "out", "sti", "tic", "oun", "enc", "ore", "ant", "ity", "fro", "art",
        "tur", "par", "red", "oth", "eri", "hic", "ies", "ste", "ght", "ich", "igh", "und", "you",
        "ort", "era", "wer", "nti", "oul", "nde", "ind", "tho", "hou", "nal", "but", "hav", "uld",
        "use", "han", "hin", "een", "ces", "cou", "lat", "tor", "ese", "age", "ame", "rin", "anc",
        "ten", "hen", "min", "eas", "can", "lit", "cha", "ous", "eat", "end", "ssi", "ial", "les",
        "ren", "tiv", "nts", "whe", "tat", "abl", "dis", "ran", "wor", "rou", "lin", "had", "sed",
        "ont", "ple", "ugh", "inc", "sio", "din", "ral", "ust", "tan", "nat", "ins", "ass", "pla",
        "ven", "ell", "she", "ose", "ite", "lly", "rec", "lan", "ard", "hey", "rie", "pos", "eme",
        "mor", "den", "oug", "tte", "ned", "rit", "ime", "sin", "ast", "any", "orm", "ndi", "ona",
        "spe", "ene", "hei", "ric", "ice", "ord", "omp", "nes", "sen", "tim", "tri", "ern", "tes",
        "por", "app", "lar", "ntr",
    ];
    trigrams.iter().map(|s| s.to_string()).collect()
}

// Returns a Vec<String> of the most common tetragrams in the English language.
pub fn get_tetragrams() -> Vec<String> {
    let tetragrams = vec![
        "tion", "atio", "that", "ther", "with", "ment", "ions", "this", "here", "from", "ould", "ting",
        "hich", "whic", "ctio", "ence", "have", "othe", "ight", "sion", "ever", "ical", "they", "inte",
        "ough", "ance", "were", "tive", "over", "ding", "pres", "nter", "comp", "able", "heir", "thei",
        "ally", "ated", "ring", "ture", "cont", "ents", "cons", "rati", "thin", "part", "form", "ning",
        "ecti", "some", "port", "enti", "onal", "itio", "hing", "ound", "will", "reat", "comm", "nder",
        "time", "emen", "iona", "more", "stat", "stan", "king", "been", "ress", "thou", "when", "acti",
        "lati", "them", "spec", "very", "side", "thes", "woul", "tain", "nati", "rate", "cent", "ount",
        "rese", "sing", "what", "tate", "even", "work", "unde", "mber", "hese", "cial", "than", "eral",
        "ater", "tter", "sent", "fore", "ract", "ling", "cess", "inst", "erat", "coun", "ange", "ties",
        "cati", "late", "mple", "into", "each", "dent", "serv", "only", "abou", "tati", "reas", "ious",
        "ssio", "most", "esti", "ness", "ctiv", "lity", "icat", "ster", "ered", "fere", "ates", "lect",
        "such", "arti", "indi", "ffer", "stra", "ings", "bout", "rent", "eren", "atte", "ener", "atur",
        "roug", "land", "come", "soci", "know", "also", "llow", "rest", "vers", "chan", "ativ", "ving",
        "ined", "call", "pers", "essi", "efor", "like", "gene", "diff", "self", "ause", "prov", "ries",
        "ilit", "ffic", "iden", "stor", "evel", "peri", "then", "long", "cond", "fect", "caus", "rica",
        "hose", "year", "utio", "esen", "ject", "rodu", "houg", "thro", "oduc", "irst", "cted", "afte",
        "tern", "conc", "fter", "firs", "tabl", "char", "once", "enta",
    ];
    tetragrams.iter().map(|s| s.to_string()).collect()
}

// Returns a Vec<String> of the most common words in the English language.
pub fn get_wordlist() -> Vec<String> {
    let wordlist = vec![
        "the", "be", "of", "and", "a", "to", "in", "he", "have", "it", "that", "for", "they", "I",
        "with", "as", "not", "on", "she", "at", "by", "this", "we", "you", "do", "but", "from", "or",
        "which", "one", "would", "all", "will", "there", "say", "who", "make", "when", "can", "more",
        "if", "no", "man", "out", "other", "so", "what", "time", "up", "go", "about", "than", "into",
        "could", "state", "only", "new", "year", "some", "take", "come", "these", "know", "see", "use",
        "get", "like", "then", "first", "any", "work", "now", "may", "such", "give", "over", "think",
        "most", "even", "find", "day", "also", "after", "way", "many", "must", "look", "before",
        "great", "back", "through", "long", "where", "much", "should", "well", "people", "down", "own",
        "just", "because", "good", "each", "those", "feel", "seem", "how", "high", "too", "place",
        "little", "world", "very", "still", "nation", "hand", "old", "life", "tell", "write", "become",
        "here", "show", "house", "both", "between", "need", "mean", "call", "develop", "under", "last",
        "right", "move", "thing", "general", "school", "never", "same", "another", "begin", "while",
        "number", "part", "turn", "real", "leave", "might", "want", "point", "form", "off", "child",
        "few", "small", "since", "against", "ask", "late", "home", "interest", "large", "person",
        "end", "open", "public", "follow", "during", "present", "without", "again", "hold", "govern",
        "around", "possible", "head", "consider", "word", "program", "problem", "however", "lead",
        "system", "set", "order", "eye", "plan", "run", "keep", "face", "fact", "group", "play",
        "stand", "increase", "early", "course", "change", "help", "line",
    ];
    wordlist.iter().map(|s| s.to_string()).collect()
}

// Returns a Vec<String> of ngrams from a comma separated file.
// The file must include no spaces between the commas. Line breaks are allowed.
//
// # Example file content:
// ```
// the,and,ing,ion,tio,ent,
// all,ons
// ```
pub fn get_from_file(path: String) -> Vec<String> {
    let file = std::fs::read_to_string(path).unwrap();
    let mut ngrams: Vec<String> = Vec::new();
    for line in file.lines() {
        for ngram in line.split(",") {
            ngrams.push(ngram.to_string());
        }
    }
    ngrams
}
