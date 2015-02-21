#![feature(plugin)]
#![allow(unstable)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

use std::io;
use std::collections::HashMap;


#[cfg(not(test))]
/// * TODO: Separate library functionality into separate crate from executable
/// * consume training file on commandline
/// * read _one_ word per line from STDIN
/// * ``for each'' word from STDIN
///     - print's a line consisting of:
///         * just the word if correctly spelled
///         * the word and the best improvement
///         * ``-'' if there aren't any improvements
fn main() {
    println!("Hello, world!");
    // open up training file given on command line
    //  - error out if no file given
    // 
    // for each line in STDIN:
    //  - read ONE word from line
    //  - print a line consisting of one of: (e.g. ``println!("{}", spell_check(word));'')
    //      * just the word if correctly spelled
    //      * the word and the best improvement
    //      * ``-'' if there aren't any improvements


    //let text: String = try!(buff.read_to_string());
}



// TODO: return/create iterator
//fn words<R: Reader>(mut buff: io::BufferedReader<R>) -> regex::RegexSplits<'r, 't> {
//fn words<'r, 't>(text: String) -> regex::RegexSplits<'r, 't> {


/// Takes in a String, returns a Vector of Strings of the words in the text
/// //fn word_indicies(text: String) -> Vec<(usize, usize)> {
fn words(text: String) -> Vec<String> {
    //let re: Box<regex::Regex> = Box::new(regex!(r"[a-z]+"));
    let re: Box<regex::Regex> = Box::new(regex!(r"[a-z]+"));
    let lowercase_text: Box<String> = Box::new(text.as_slice()
                                                   .chars()
                                                   .map(|c| c.to_lowercase())
                                                   .collect::<String>());
    //let word_slices: regex::FindMatches = re.find_iter(lowercase_text.as_slice());
    let mut words: Vec<String> = vec![];
    let lowercase_text_slice: &str = lowercase_text.as_slice();
    for (start, end) in re.find_iter(lowercase_text.as_slice()) {
        words.push(lowercase_text_slice.slice(start, end).to_string())
    }
    //let words: Vec<(usize, usize)> = word_slices.collect();
    words
}

#[cfg(test)]
mod words_tests {
    use super::words;
    use std::io;
    use regex;

    #[test]
    fn splits_string_into_words() {
        let e: Vec<String> = vec!["these".to_string(), "are".to_string(), "some".to_string(), "words".to_string()];
        //let e: Vec<(usize, usize)> = vec![(0, 5), (6, 9), (10, 14), (15, 20)];
        words_expect("these are some words", e);
    }

    #[test]
    fn treats_apostrophes_as_word_separators() {
        let e: Vec<String> = vec!["these".to_string(), "aren".to_string(), "t".to_string(), "the".to_string(), "same".to_string(), "words".to_string()];
        //let e: Vec<(usize, usize)> = vec![(0, 5), (6, 10), (11, 12), (13, 16), (17, 21), (22, 27)];
        words_expect("these aren't the same words", e);
    }

    #[test]
    fn ignores_special_characters() {
        let e: Vec<String> = vec!["lonely".to_string()];
        //let e: Vec<(usize, usize)> = vec![(9, 15)];
        words_expect("$#%6(&^\t\nlonely\t\r$%^", e);
    }

    fn words_expect(s: &str, e: Vec<String>) {
        assert_eq!(words(s.to_string()), e);
    }

    fn mk_reader(s: &str) -> io::BufferedReader<io::MemReader> {
        let bytes = s.to_string().into_bytes();
        io::BufferedReader::new(io::MemReader::new(bytes))
    }
}


/// training module
/// * computes frequencies of correctly spelled words in given corpus
/// TODO: Make an actual submodule, i.e. accessible via ``spelling_corrector::training'' with
/// code available in ``src/training.rs'' or ``src/training/mod.rs''
// won't necessarily take a String, may take BufferedReader or maybe &String, or...
//fn train(corpus: String) -> HashMap<String, usize> {
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
    use std::io;
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

/// spell check module
/// * TODO: Make actual submodule, i.e. accessible via ``spelling_corrector::spell_check'' with
/// code available in ``src/spell_check.rs'' or ``src/spell_check/mod.rs''
/// * uses resulting frequencies from training to check individual words
///     - ex: ``check(word, known_words)''
/// * checks whether or not given word spelled correctly according to training module
///     - if not, checks whether ``small edits'' create a correctly spelled variant
///         * ``small edits'' means application of _one_ edit action __possibly__ followed by the
///         application of a _second one_ to the result of the first
/// * once all possible candidates generated, picks _most frequently_ used from training corpus
///     - if none of candidates correct word, failure reported

/// edit actions module
/// * TODO: Make actual submodule, i.e. accessible via ``spelling_corrector::edit_actions'' with
/// code available in ``src/edit_actions.rs'' or ``src/edit_actions/mod.rs''
/// * given a word, an edit action is one of the following
///     - deletion of _one_ letter
///     - transposition of _two_ neighboring letters
///     - replacement of _one_ letter with another
///     - insertion of a letter at any position

/*fn edits1(word: String) -> ... {

    //splits = [(word[:i], word[i:]), for i in range(len(word) + 1)]
    let mut splits: Vec<(String, String)> = vec![];
    for i in range_inclusive(0, word.len()) {
        splits.push((word.as_slice().slice_to(i), word.as_slice().slice_from(i)))
    }
}*/

fn splits(word: String) -> Vec<(String, String)> {
    let mut splits: Vec<(String, String)> = vec![];
    for i in range(0, word.len()+1) {
        let word_split: (String, String) = (word.as_slice().slice_to(i).to_string(),
                                            word.as_slice().slice_from(i).to_string());
        splits.push(word_split)
    }
    splits
}
#[cfg(test)]
mod splits_tests {

    use super::splits;
    use std::io;

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
        assert_eq!(splits(s.to_string()), e);
    }

}

fn deletes(word: String, splits: &Vec<(String, String)>) -> Vec<String> {
    let mut deletes: Vec<String> = vec![];
    for &(ref a, ref b) in splits.iter() {
        if !b.is_empty() {
            let b_minus_first_char: &str = b.as_slice().slice_from(1);
            let deleted: String = a.to_string() + b_minus_first_char;
            deletes.push(deleted)
        }
    }
    deletes
}

#[cfg(test)]
mod deletes_tests {
    use super::deletes;
    use super::splits;
    use std::io;

    #[test]
    fn deletes_empty_string() {
        let e: Vec<String> = vec![];
        deletes_expect("", e);
    }

    #[test]
    fn deletes_single_character_string() {
        let e: Vec<String> = vec!["".to_string()];
        deletes_expect("a", e);
    }

    #[test]
    fn deletes_chars_from_word() {
        let e: Vec<String> = vec!["he".to_string(), "te".to_string(), "th".to_string()];
        deletes_expect("the", e);
    }

    fn deletes_expect(s: &str, e: Vec<String>) {
        let boxed_splits = Box::new(splits(s.to_string()));
        assert_eq!(deletes(s.to_string(),&*boxed_splits), e);
    }
}

fn transposes(word: String, splits: &Vec<(String, String)>) -> Vec<String> {
    let mut transposes: Vec<String> = vec![];
    for &(ref a, ref b) in splits.iter() {
        if b.len() > 1 {
            let b_sub_1: &str = b.as_slice().slice(1,2);
            let b_sub_0: &str = b.as_slice().slice(0,1);
            let rest_of_b: &str = b.as_slice().slice_from(2);
            let transposed: String = a.to_string() + b_sub_1 + b_sub_0 + rest_of_b;
            transposes.push(transposed)
        }
    }
    transposes
}

#[cfg(test)]
mod transposes_tests {
    use super::transposes;
    use super::splits;
    use std::io;

    #[test]
    fn transposes_empty_string() {
        let e: Vec<String> = vec![];
        transposes_expect("", e);
    }

    #[test]
    fn transposes_single_character_string() {
        let e: Vec<String> = vec![];
        transposes_expect("a", e);
    }

    #[test]
    fn transposes_words() {
        let e: Vec<String> = vec!["hte".to_string(), "teh".to_string()];
        transposes_expect("the", e);
    }

    fn transposes_expect(s: &str, e: Vec<String>) {
        let boxed_splits: Box<Vec<(String, String)>> = Box::new(splits(s.to_string()));
        assert_eq!(transposes(s.to_string(),&*boxed_splits), e);
    }

}

fn replaces(word: String, splits: Vec<(String, String)>) -> Vec<String> {
    let mut replaces: Vec<String> = vec![];
    let alphabet: Vec<&str> = vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p",
                                   "q","r","s","t","u","v","w","x","y","z"];
    for &(a, b) in splits.iter() {
        for c in alphabet.iter() {
            if !b.is_empty() {
                let replaced: String = (a + *c + b.as_slice().slice_from(1)).to_string();
                replaces.push(replaced)
            }
        }
    }
    replaces
}

#[cfg(test)]
mod replaces_tests {
    use super::replaces;
    use super::splits;

    #[test]
    fn replaces_empty_string() {
        let e: Vec<String> = vec![];
        replaces_expect("", e);
    }

    #[test]
    fn replaces_single_character_string() {
        let e: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(),
                                  "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(),
                                  "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(),
                                  "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(),
                                  "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(),
                                  "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(),
                                  "y".to_string(), "z".to_string()];
        replaces_expect("a", e);
    }

    #[test]
    fn replaces_characters_in_word() {
        let replaced = replaces("hi".to_string(), splits("hi".to_string()));
        assert_eq!(replaced.len(), 52);
    }

    fn replaces_expect(s: &str, e: Vec<String>) {
        assert_eq!(replaces(s.to_string(),splits(s.to_string())), e);
    }
}



/// Python Version of spell check algorithm detailed above
///
/// ```python
/// import re, collections
///
/// def words(text): return re.findall('[a-z]+', text.lower())
///
/// def train(features):
///     model = collections.defaultdict(lambda: 1) # get() -> Option (match None => 1)
///     for f in features:
///         model[f] += 1
///     return model
///
/// NWORDS = train(words(file('big.txt').read()))
///
/// alphabet = 'abcdefghijklmnopqrstuvwxyz'
///
/// def edits1(word):
///     splits     = [(word[:i], word[i:]) for i in range(len(word) + 1)]
///     deletes    = [a + b[1:] for a, b in splits if b]
///     transposes = [a + b[1] + b[0] + b[2:] for a, b in splits if len(b)>1]
///     replaces   = [a + c + b[1:] for a, b in splits for c in alphabet if b]
///     inserts    = [a + c + b     for a, b in splits for c in alphabet]
///     return set(deletes + transposes + replaces + inserts)
///
/// def known_edits2(word):
///     return set(e2 for e1 in edits1(word) for e2 in edits1(e1) if e2 in NWORDS)
///
/// def known(words): return set(w for w in words if w in NWORDS)
///
/// def correct(word):
///     candidates = known([word]) or known(edits1(word)) or known_edits2(word) or [word]
///     return max(candidates, key=NWORDS.get)
/// ```
fn foobar() -> () {} // necessary so rustc doesn't complain
