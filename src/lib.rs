use pyo3::prelude::*;
use rayon::prelude::*;
use regex::Regex;

const RE_WORDS: &str = r"\w+|[^\w]|[\s]";

// fn load_regexen() -> HashMap<&str, Regex> {
//     let mut regexen = HashMap::new();
//     regexen.insert("words",  Regex::new(RE_WORDS).unwrap());
//     Ok(regexen)
// }

// Extract all words from the given text
fn split_words(text: &str) -> Vec<&str> {
    let re_words = Regex::new(RE_WORDS).unwrap();
    re_words.find_iter(text).map(|m| m.as_str()).collect()
}

fn process_word(word: &str, affix: &str, postfix: &str) -> String {
    let mid_word = word.len() / 2;
    let start = &word[..mid_word];
    let end = &word[mid_word..];

    format!("{}{}{}{}", affix, start, postfix, end)
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

        let result: Vec<_> = split_words(text);

        let expected: usize = 22;
        assert_eq!(expected, result.len());
    }
}
