//! find possible corrections for misspelled words using statistical language processing, based on
//! the spelling corrector outlined by Norvig (see: http://norvig.com/spell-correct.html)

#![feature(plugin)]
#![allow(unstable)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

use std::io;
use std::os;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(not(test))]
// TODO: Separate library functionality into separate crate from executable
fn main() {

    let training_file = match os::args().tail().first() {
        Some(arg) => io::File::open(&Path::new(arg)),
        None      => panic!("Must provide training file!"),
    };
    let mut training_buff = io::BufferedReader::new(training_file);

    let text: String = training_buff.read_to_string().unwrap();

    let words_vec: Vec<String> = words(text);

    let trained: Box<HashMap<String, usize>> = Box::new(train(words_vec));

    let re: Box<regex::Regex> = Box::new(regex!(r"[a-z]+"));
    for line in io::stdin().lock().lines() {
        let line_text: String = line.unwrap();

        if let Some((start, end)) = re.find(line_text.as_slice()) {
            let word: &str = line_text.slice(start, end);
            let improved: String = correct(word, &*trained);
            if word == improved.as_slice() {
                println!("{}", word);
            } else {
                println!("{}, {}", word, improved);
            }
        }
    }
}


/// Given a `String`, extracts the words from within into a Vector of Strings
fn words(text: String) -> Vec<String> {
    let re: Box<regex::Regex> = Box::new(regex!(r"[a-z]+"));
    let lowercase_text: Box<String> = Box::new(text.as_slice()
                                                   .chars()
                                                   .map(|c| c.to_lowercase())
                                                   .collect::<String>());
    let mut words: Vec<String> = vec![];
    let lowercase_text_slice: &str = lowercase_text.as_slice();
    for (start, end) in re.find_iter(lowercase_text.as_slice()) {
        words.push(lowercase_text_slice.slice(start, end).to_string())
    }
    words
}

#[cfg(test)]
mod words_tests {
    use super::words;

    #[test]
    fn splits_string_into_words() {
        let e: Vec<String> = vec!["these".to_string(), "are".to_string(), "some".to_string(),
                                  "words".to_string()];
        words_expect("these are some words", e);
    }

    #[test]
    fn treats_apostrophes_as_word_separators() {
        let e: Vec<String> = vec!["these".to_string(), "aren".to_string(), "t".to_string(),
                                  "the".to_string(), "same".to_string(), "words".to_string()];
        words_expect("these aren't the same words", e);
    }

    #[test]
    fn ignores_special_characters() {
        let e: Vec<String> = vec!["lonely".to_string()];
        words_expect("$#%6(&^\t\nlonely\t\r$%^", e);
    }

    fn words_expect(s: &str, e: Vec<String>) {
        assert_eq!(words(s.to_string()), e);
    }
}


/// training module
/// Given a list of words, builds a HashMap of the frequencies of the words.
fn train(corpus: Vec<String>) -> HashMap<String, usize> {
    let freqs: HashMap<String, usize> = HashMap::new();
    corpus.iter().fold(freqs, |mut acc: HashMap<String, usize>, word: &String| {
        let freq: usize = acc.get(word.as_slice()).map_or(1, |f| *f + 1);
        acc.insert((*word.as_slice()).to_string(), freq);
        acc
    })
}

#[cfg(test)]
mod train_tests {
    use super::train;
    use std::collections::HashMap;

    #[test]
    fn counts_no_words() {
        let expected: HashMap<String, usize> = HashMap::new();
        assert_eq!(train(vec![]), expected);
    }

    #[test]
    fn counts_single_word() {
        let mut e: HashMap<String, usize> = HashMap::new();
        e.insert("foo".to_string(), 1);
        assert_eq!(train(vec!["foo".to_string()]), e);
    }

    #[test]
    fn counts_multiple_words() {
        let mut e: HashMap<String, usize> = HashMap::new();
        e.insert("foo".to_string(), 2);
        assert_eq!(train(vec!["foo".to_string(), "foo".to_string()]), e);
    }

    #[test]
    fn counts_different_words() {
        let mut e: HashMap<String, usize> = HashMap::new();
        e.insert("foo".to_string(), 2);
        e.insert("bar".to_string(), 1);
        assert_eq!(train(vec!["foo".to_string(), "bar".to_string(), "foo".to_string()]), e);
    }
}

/// edit actions module
/// returns a set of strings from performing edit actions on the given word, where `edit action' is
/// defined as one of the following:
///     - deletion of _one_ letter
///     - transposition of _two_ neighboring letters
///     - replacement of _one_ letter with another
///     - insertion of a letter at any position
fn edits1(word: &str) -> HashSet<String> {

    let word_splits: Box<Vec<(String, String)>> = Box::new(splits(word));

    let deletes: HashSet<String> = deletes(&*word_splits);

    let transposes: HashSet<String> = transposes(&*word_splits);

    let replaces: HashSet<String> = replaces(&*word_splits);

    let inserts: HashSet<String> = inserts(&*word_splits);

    let mut edit_set: HashSet<String> = deletes.union(&transposes).map(|&ref x| x.to_string()).collect();
    edit_set = edit_set.union(&replaces).map(|&ref x| x.to_string()).collect();
    edit_set = edit_set.union(&inserts).map(|&ref x| x.to_string()).collect();

    edit_set
}

#[cfg(test)]
mod edits1_tests {
    use super::edits1;
    use std::collections::HashSet;

    #[test]
    fn builds_set_of_edits() {
        let alphabet: [String; 26] = ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(),
                                      "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(),
                                      "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(),
                                      "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(),
                                      "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(),
                                      "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(),
                                      "y".to_string(), "z".to_string()];
        let e: HashSet<String> = alphabet.iter().map(|&ref x| x.to_string()).collect();
        assert_eq!(edits1(""), e);
        assert_eq!(edits1("a").len(), 78);
    }
}

/// computes the 2nd edit distance from the given set of first edit distances, only keeping those
/// candidates that are actually known to be in the given training dictionary. Returns `None` if
/// the given set is empty, otherwise returns `Some(HashSet<String>)`
fn known_edits2(first_edits: &HashSet<String>, nwords: &HashMap<String, usize>) -> Option<HashSet<String>> {
    let mut known_edits2: HashSet<String> = HashSet::new();
    for e1 in first_edits.iter() {
        let edits2: HashSet<String> = edits1(e1.as_slice());
        for e2 in edits2.iter() {
            if nwords.contains_key(e2) {
                known_edits2.insert(e2.to_string());
            }
        }
    }
    if known_edits2.len() == 0 { None } else { Some(known_edits2) }
}

#[cfg(test)]
mod known_edits2_tests {
    use super::known_edits2;
    use super::edits1;
    use std::collections::HashSet;
    use std::collections::HashMap;
    
    #[test]
    fn depends_on_nwords() {
        let first_edits: Box<HashSet<String>> = Box::new(HashSet::new());
        let nwords: HashMap<String, usize> = HashMap::new();
        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);
        assert!(known_edits2(&*first_edits, &*boxed_nwords).is_none());
    }

    #[test]
    fn computes_two_level_edits() {
        let first_edits: Box<HashSet<String>> = Box::new(edits1(""));
        let mut nwords: HashMap<String, usize> = HashMap::new();
        let mut e: HashSet<String> = HashSet::new();
        e.insert("a".to_string()); e.insert("xx".to_string()); e.insert("mr".to_string());

        nwords.insert("a".to_string(), 1000);
        nwords.insert("xx".to_string(), 500);
        nwords.insert("mr".to_string(), 300);

        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);

        assert_eq!(known_edits2(&*first_edits, &*boxed_nwords).unwrap(), e);
    }
}

/// Given a set of possible words and the training dictionary, builds a set of those words that are
/// contained in the training dictionary. Returns `None` if none of the given words are in the
/// training dictionary, otherwise returns `Some(HashSet<String>)`
fn known(words: &HashSet<String>, nwords: &HashMap<String, usize>) -> Option<HashSet<String>> {
    let mut known_set: HashSet<String> = HashSet::new();
    for w in words.iter() {
        if nwords.contains_key(w) {
            known_set.insert(w.to_string());
        }
    }
    if known_set.len() == 0 { None } else { Some(known_set) }
}

#[cfg(test)]
mod known_tests {
    use super::known;
    use std::collections::HashSet;
    use std::collections::HashMap;

    #[test]
    fn returns_none_if_word_unknown() {
        let nwords: HashMap<String, usize> = HashMap::new();
        let mut words: HashSet<String> = HashSet::new();
        words.insert("these".to_string()); words.insert("are".to_string());
        words.insert("some".to_string()); words.insert("words".to_string());
        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);
        let boxed_words: Box<HashSet<String>> = Box::new(words);
        assert!(known(&*boxed_words, &*boxed_nwords).is_none());
    }

    #[test]
    fn returns_known_valid_words() {
        let mut nwords: HashMap<String, usize> = HashMap::new();
        nwords.insert("words".to_string(), 5);

        let mut words: HashSet<String> = HashSet::new();
        words.insert("these".to_string()); words.insert("are".to_string());
        words.insert("some".to_string()); words.insert("words".to_string());

        let mut e: HashSet<String> = HashSet::new();
        e.insert("words".to_string());
        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);
        let boxed_words: Box<HashSet<String>> = Box::new(words);

        assert_eq!(known(&*boxed_words, &*boxed_nwords).unwrap(), e);
    }
}

/// Given a word and training dictionary, attempts to find the best spelling improvement for said
/// word. If no improvement can be found, returns "-". The model implemented is:
/// - all known words of edit distance 1 are infinitely more probable than known words of edit
/// distance 2 and infinitely less probable than a known word of edit distance 0
fn correct(word: &str, nwords: &HashMap<String, usize>) -> String {
    let mut word_set: HashSet<String> = HashSet::new();
    word_set.insert(word.to_string());

    let boxed_word_set: Box<HashSet<String>> = Box::new(word_set);

    if let Some(candidates) = known(&*boxed_word_set, nwords) {
        return best_guess(&candidates, nwords)
    }

    let first_edits: HashSet<String> = edits1(word);
    let boxed_first_edits: Box<HashSet<String>> = Box::new(first_edits);

    if let Some(candidates) = known(&*boxed_first_edits, nwords) {
        return best_guess(&candidates, nwords)
    }

    if let Some(candidates) = known_edits2(&*boxed_first_edits, nwords) {
        return best_guess(&candidates, nwords)
    }

    best_guess(&*boxed_word_set, nwords)
}

#[cfg(test)]
mod correct_tests {
    use super::correct;
    use std::collections::HashMap;


    #[test]
    fn finds_best_improvement() {
        let mut nwords: HashMap<String, usize> = HashMap::new();
        nwords.insert("apple".to_string(), 2); nwords.insert("banana".to_string(), 3);
        nwords.insert("canada".to_string(), 1); nwords.insert("manna".to_string(), 2);
        nwords.insert("the".to_string(), 42);
        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);

        assert_eq!(correct("asdf", &*boxed_nwords).as_slice(), "-");
        assert_eq!(correct("teh", &*boxed_nwords).as_slice(), "the");
        assert_eq!(correct("banana", &*boxed_nwords).as_slice(), "banana");
        assert_eq!(correct("aoplr", &*boxed_nwords).as_slice(), "apple");
    }

}

/// Given a set of candidate words and a training dictionary, returns the candidate word that has
/// the highest frequency. If no candidate could be found, returns `"-"`. 
fn best_guess(candidates: &HashSet<String>, nwords: &HashMap<String, usize>) -> String {
    candidates.iter().fold("-".to_string(), |acc, &ref c| {
        let best_so_far: Option<&usize> = nwords.get(&acc);
        let candidate: Option<&usize> = nwords.get(c);

        match candidate {
            Some(v) => match best_so_far {
                Some(val) => if v <= val { acc } else { c.to_string() },
                None      => c.to_string(),
            },
            None    => acc,
        }
    })
}

#[cfg(test)]
mod best_guess_tests {
    use super::best_guess;
    use std::collections::{HashSet,HashMap};

    #[test]
    fn returns_dash_for_no_matches() {
        let mt_cands: HashSet<String> = HashSet::new();
        let mt_nwords: HashMap<String, usize> = HashMap::new();

        let boxed_mt_cands: Box<HashSet<String>> = Box::new(mt_cands);
        let boxed_mt_nwords: Box<HashMap<String, usize>> = Box::new(mt_nwords);
        assert_eq!(best_guess(&*boxed_mt_cands, &*boxed_mt_nwords).as_slice(), "-");

        let mut nwords: HashMap<String, usize> = HashMap::new();
        nwords.insert("apple".to_string(), 2);

        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);

        assert_eq!(best_guess(&*boxed_mt_cands, &*boxed_nwords).as_slice(), "-");

        let mut cands: HashSet<String> = HashSet::new();
        cands.insert("banana".to_string());

        let boxed_cands: Box<HashSet<String>> = Box::new(cands);

        assert_eq!(best_guess(&*boxed_cands, &*boxed_nwords).as_slice(), "-");
    }

    #[test]
    fn returns_best_guess() {
        let mut nwords: HashMap<String, usize> = HashMap::new();
        nwords.insert("apple".to_string(), 2); nwords.insert("banana".to_string(), 3);
        nwords.insert("canada".to_string(), 1); nwords.insert("manna".to_string(), 2);

        let mut cands: HashSet<String> = HashSet::new();
        cands.insert("banana".to_string());
        cands.insert("canada".to_string());
        cands.insert("havana".to_string());
        cands.insert("panama".to_string());
        let boxed_cands: Box<HashSet<String>> = Box::new(cands);
        let boxed_nwords: Box<HashMap<String, usize>> = Box::new(nwords);
        assert_eq!(best_guess(&*boxed_cands, &*boxed_nwords).as_slice(), "banana");
    }
}

/// Given a word, computes the splits for said word
///
/// # Example:
///
/// ```rust
/// let e: Vec<(String, String)> = vec![("".to_string(), "the".to_string()),
///                                     ("t".to_string(), "he".to_string()),
///                                     ("th".to_string(), "e".to_string()),
///                                     ("the".to_string(), "".to_string())];
/// assert_eq!(splits("the"), e);
/// ```
fn splits(word: &str) -> Vec<(String, String)> {
    let mut splits: Vec<(String, String)> = vec![];
    for i in range(0, word.len()+1) {
        let word_split: (String, String) = (word.slice_to(i).to_string(),
                                            word.slice_from(i).to_string());
        splits.push(word_split)
    }
    splits
}

#[cfg(test)]
mod splits_tests {
    use super::splits;

    #[test]
    fn splits_empty_string() {
        let e: Vec<(String, String)> = vec![("".to_string(), "".to_string())];
        splits_expect("", e);
    }

    #[test]
    fn splits_single_character_string() {
        let e: Vec<(String, String)> = vec![("".to_string(), "a".to_string()),
                                            ("a".to_string(), "".to_string())];
        splits_expect("a", e);
    }

    #[test]
    fn splits_word() {
        let e: Vec<(String, String)> = vec![("".to_string(), "the".to_string()),
                                            ("t".to_string(), "he".to_string()),
                                            ("th".to_string(), "e".to_string()),
                                            ("the".to_string(), "".to_string())];
        splits_expect("the", e);
    }

    fn splits_expect(s: &str, e: Vec<(String, String)>) {
        assert_eq!(splits(s), e);
    }

}

/// Given a list of word splits, deletes the first letter of the second of each split pair.
fn deletes(splits: &Vec<(String, String)>) -> HashSet<String> {
    let mut deletes: HashSet<String> = HashSet::new();
    for &(ref a, ref b) in splits.iter() {
        if !b.is_empty() {
            let b_minus_first_char: &str = b.as_slice().slice_from(1);
            let deleted: String = a.to_string() + b_minus_first_char;
            deletes.insert(deleted);
        }
    }
    deletes
}

#[cfg(test)]
mod deletes_tests {
    use super::deletes;
    use super::splits;
    use std::collections::HashSet;

    #[test]
    fn deletes_empty_string() {
        let e: HashSet<String> = HashSet::new();
        deletes_expect("", e);
    }

    #[test]
    fn deletes_single_character_string() {
        let mut e: HashSet<String> = HashSet::new();
        e.insert("".to_string());
        deletes_expect("a", e);
    }

    #[test]
    fn deletes_chars_from_word() {
        let mut e: HashSet<String> = HashSet::new();
        e.insert("he".to_string()); e.insert("te".to_string()); e.insert("th".to_string());
        deletes_expect("the", e);
    }

    fn deletes_expect(s: &str, e: HashSet<String>) {
        let boxed_splits = Box::new(splits(s));
        assert_eq!(deletes(&*boxed_splits), e);
    }
}

/// Given a list of word splits, builds a set of the transpositions of two neighboring letters for
/// each split pair.
fn transposes(splits: &Vec<(String, String)>) -> HashSet<String> {
    let mut transposes: HashSet<String> = HashSet::new();
    for &(ref a, ref b) in splits.iter() {
        if b.len() > 1 {
            let b_sub_1: &str = b.as_slice().slice(1,2);
            let b_sub_0: &str = b.as_slice().slice(0,1);
            let rest_of_b: &str = b.as_slice().slice_from(2);
            let transposed: String = a.to_string() + b_sub_1 + b_sub_0 + rest_of_b;
            transposes.insert(transposed);
        }
    }
    transposes
}

#[cfg(test)]
mod transposes_tests {
    use super::transposes;
    use super::splits;
    use std::collections::HashSet;

    #[test]
    fn transposes_empty_string() {
        let e: HashSet<String> = HashSet::new();
        transposes_expect("", e);
    }

    #[test]
    fn transposes_single_character_string() {
        let e: HashSet<String> = HashSet::new();
        transposes_expect("a", e);
    }

    #[test]
    fn transposes_words() {
        let mut e: HashSet<String> = HashSet::new();
        e.insert("hte".to_string()); e.insert("teh".to_string());
        transposes_expect("the", e);
    }

    fn transposes_expect(s: &str, e: HashSet<String>) {
        let boxed_splits: Box<Vec<(String, String)>> = Box::new(splits(s));
        assert_eq!(transposes(&*boxed_splits), e);
    }

}

/// Given list of word splits, computes the set of replacing one of the letters in the second of
/// the split pair.
fn replaces(splits: &Vec<(String, String)>) -> HashSet<String> {
    let mut replaces: HashSet<String> = HashSet::new();
    let alphabet: [&str; 26] = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p",
                                "q","r","s","t","u","v","w","x","y","z"];
    for &(ref a, ref b) in splits.iter() {
        for c in alphabet.iter() {
            if !b.is_empty() {
                let rest_of_b: &str = b.as_slice().slice_from(1);
                let replaced: String = a.to_string() + *c + rest_of_b;
                replaces.insert(replaced);
            }
        }
    }
    replaces
}

#[cfg(test)]
mod replaces_tests {
    use super::replaces;
    use super::splits;
    use std::collections::HashSet;

    #[test]
    fn replaces_empty_string() {
        let e: HashSet<String> = HashSet::new();
        replaces_expect("", e);
    }

    #[test]
    fn replaces_single_character_string() {
        let mut e: HashSet<String> = HashSet::new();
        e.insert("a".to_string()); e.insert("b".to_string()); e.insert("c".to_string());
        e.insert("d".to_string()); e.insert("e".to_string()); e.insert("f".to_string());
        e.insert("g".to_string()); e.insert("h".to_string()); e.insert("i".to_string());
        e.insert("j".to_string()); e.insert("k".to_string()); e.insert("l".to_string());
        e.insert("m".to_string()); e.insert("n".to_string()); e.insert("o".to_string());
        e.insert("p".to_string()); e.insert("q".to_string()); e.insert("r".to_string());
        e.insert("s".to_string()); e.insert("t".to_string()); e.insert("u".to_string());
        e.insert("v".to_string()); e.insert("w".to_string()); e.insert("x".to_string());
        e.insert("y".to_string()); e.insert("z".to_string());
        replaces_expect("a", e);
    }

    #[test]
    fn replaces_characters_in_word() {
        let boxed_splits: Box<Vec<(String, String)>> = Box::new(splits("hi"));
        let replaced: HashSet<String> = replaces(&*boxed_splits);
        assert_eq!(replaced.len(), 51);
    }

    fn replaces_expect(s: &str, e: HashSet<String>) {
        let boxed_splits: Box<Vec<(String, String)>> = Box::new(splits(s));
        assert_eq!(replaces(&*boxed_splits), e);
    }
}

/// Given list of word splits, computes the set of inserting a letter at any position for each of
/// the split pairs.
fn inserts(splits: &Vec<(String, String)>) -> HashSet<String> {
    let mut inserts: HashSet<String> = HashSet::new();
    let alphabet: [&str; 26] = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p",
                                "q","r","s","t","u","v","w","x","y","z"];
    for &(ref a, ref b) in splits.iter() {
        for c in alphabet.iter() {
            let inserted: String = a.to_string() + *c + b.as_slice();
            inserts.insert(inserted);
        }
    }
    inserts
}

#[cfg(test)]
mod inserts_tests {
    use super::inserts;
    use super::splits;
    use std::collections::HashSet;

    #[test]
    fn inserts_empty_string() {
        let ex: [String; 26] = ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(),
                                "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(),
                                "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(),
                                "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(),
                                "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(),
                                "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(),
                                "y".to_string(), "z".to_string()];
        let e: HashSet<String> = ex.iter().map(|&ref x| x.to_string()).collect();
        inserts_expect("", e);

    }

    #[test]
    fn inserts_chars_in_word() {
        let boxed_splits: Box<Vec<(String, String)>> = Box::new(splits("hi"));
        let inserted = inserts(&*boxed_splits);
        assert_eq!(inserted.len(), 76);
    }

    fn inserts_expect(s: &str, e: HashSet<String>) {
        let boxed_splits: Box<Vec<(String, String)>> = Box::new(splits(s));
        assert_eq!(inserts(&*boxed_splits), e);
    }

}
