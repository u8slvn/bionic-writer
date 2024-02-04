use pyo3::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REGEXEN: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();
        m.insert("WORDS", Regex::new(r"\w+|\W|\s").unwrap());
        m.insert("WHITESPACE", Regex::new(r"^\s$").unwrap());
        m.insert("NUMBER", Regex::new(r"^\d$").unwrap());
        m.insert("NON_WORD", Regex::new(r"^\W$").unwrap());
        m
    };
}

// Extract all words from the given text.
fn split_words(text: &str) -> Vec<&str> {
    let re_words = REGEXEN.get("WORDS").unwrap();
    re_words.find_iter(text).map(|m| m.as_str()).collect()
}

// Format word to bionic reading style.
fn format_word(word: &str, affix: &str, postfix: &str, index: usize) -> String {
    let start = &word[..index];
    let end = &word[index..];
    format!("{}{}{}{}", affix, start, postfix, end)
}

fn process_word(word: &str, affix: &str, postfix: &str) -> String {
    if REGEXEN.get("WHITESPACE").unwrap().is_match(word)
        || REGEXEN.get("NON_WORD").unwrap().is_match(word)
    {
        return word.to_string();
    }

    if REGEXEN.get("NUMBER").unwrap().is_match(word) {
        return format_word(word, affix, postfix, word.len());
    }

    let index = word.len() / 2;
    format_word(word, affix, postfix, index)
}

#[pyfunction]
fn write(text: &str, affix: &str, postfix: &str) -> PyResult<String> {
    let words: Vec<_> = split_words(text);
    let processed_words: Vec<_> = words
        .par_iter()
        .map(|word| process_word(word, affix, postfix))
        .collect();

    Ok(processed_words.join(""))
}

#[pymodule]
fn bionic_writer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(write, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words() {
        let text = "Your bones don't break, mine do. That's clear.";

        let result = split_words(text);

        let expected = 22;
        assert_eq!(expected, result.len());
    }

    #[test]
    fn test_format_word() {
        let word = "Hello";
        let affix = "<b>";
        let postfix = "</b>";
        let index = 2;

        let result = format_word(word, affix, postfix, index);

        let expected = "<b>He</b>llo";
        assert_eq!(expected, result);
    }
}
