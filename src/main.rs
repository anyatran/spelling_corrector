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


/// Takes in a String, returns a Vector of the start and end byte indicies for each word in the
/// given String
fn word_indicies(text: String) -> Vec<(usize, usize)> {
    let re: Box<regex::Regex> = Box::new(regex!(r"[a-z]+"));
    let lowercase_text: Box<String> = Box::new(text.as_slice()
                                                   .chars()
                                                   .map(|c| c.to_lowercase())
                                                   .collect::<String>());
    let word_slices: regex::FindMatches = re.find_iter(lowercase_text.as_slice());
    let words: Vec<(usize, usize)> = word_slices.collect();
    words
}

#[cfg(test)]
mod word_indicies_tests {
    use super::word_indicies;
    use std::io;
    use regex;

    #[test]
    fn splits_string_into_words() {
        //let e: Vec<String> = vec!["these", "are", "some", "words"];
        let e: Vec<(usize, usize)> = vec![(0, 5), (6, 9), (10, 14), (15, 20)];
        word_indicies_expect("these are some words", e);
    }

    #[test]
    fn treats_apostrophes_as_word_separators() {
        //let e: Vec<String> = vec!["these", "aren", "t", "the", "same", "words"];
        let e: Vec<(usize, usize)> = vec![(0, 5), (6, 10), (11, 12), (13, 16), (17, 21), (22, 27)];
        word_indicies_expect("these aren't the same words", e);
    }

    #[test]
    fn ignores_special_characters() {
        //let e: Vec<String> = vec!["lonely"];
        let e: Vec<(usize, usize)> = vec![(9, 15)];
        word_indicies_expect("$#%6(&^\t\nlonely\t\r$%^", e);
    }

    fn word_indicies_expect(s: &str, e: Vec<(usize, usize)>) {
        assert_eq!(word_indicies(s.to_string()), e);
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
fn train(corpus: String) -> HashMap<String, usize> {
    println!("I'm training!");
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
