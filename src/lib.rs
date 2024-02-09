use pyo3::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
extern crate utf8_slice;

lazy_static! {
    static ref REGEXEN: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();
        m.insert("WORDS", Regex::new(r"\w+|\W|\s").unwrap());
        m.insert("WHITESPACE", Regex::new(r"^\s$").unwrap());
        m.insert("NUMBER", Regex::new(r"^\d+$").unwrap());
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
    let start = utf8_slice::till(word, index);
    let end = utf8_slice::from(word, index);
    format!("{}{}{}{}", affix, start, postfix, end)
}

// Process word with bionic reading style.
fn process_word(word: &str, affix: &str, postfix: &str) -> String {
    if REGEXEN.get("WHITESPACE").unwrap().is_match(word)
        || REGEXEN.get("NON_WORD").unwrap().is_match(word)
        || REGEXEN.get("NUMBER").unwrap().is_match(word)
    {
        return word.to_string();
    }

    let word_len = utf8_slice::len(word);
    let index = if word_len == 1 { 1 } else { word_len / 2 };
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
        let affix = "[b]";
        let postfix = "[/b]";
        let index = 2;

        let result = format_word(word, affix, postfix, index);

        let expected = "[b]He[/b]llo";
        assert_eq!(expected, result);
    }

    macro_rules! test_process_word_parametrized {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (word, expected) = $value;
                let affix = "[b]";
                let postfix = "[/b]";

                let result = process_word(word, affix, postfix);

                assert_eq!(expected, result);
            }
        )*
        }
    }

    test_process_word_parametrized! {
        test_process_word_with_whitespace: (" ", " "),
        test_process_word_with_non_word: (":", ":"),
        test_process_word_with_number: ("1990", "1990"),
        test_process_word_with_utf8_word: ("déjà", "[b]dé[/b]jà"),
        test_process_word_with_word: ("Hello", "[b]He[/b]llo"),
    }

    #[test]
    fn test_write_text_as_bionic_reading() {
        let text = concat!(
            "Now that we know who you are, I know who I am. I'm not ",
            "a mistake! It all makes sense! In a comic, you know how you can tell ",
            "who the arch-villain's going to be? He's the exact opposite of the ",
            "hero. And most times they're friends, like you and me! I should've ",
            "known way back when... You know why, David? Because of the kids. They ",
            "called me Mr Glass."
        );
        let affix = "**";
        let postfix = "**";

        let result = write(text, affix, postfix);

        let expected = concat!(
            "**N**ow **th**at **w**e **kn**ow **w**ho **y**ou **a**re, **I** ",
            "**kn**ow **w**ho **I** **a**m. **I**'**m** **n**ot **a** ",
            "**mis**take! **I**t **a**ll **ma**kes **se**nse! **I**n **a** ",
            "**co**mic, **y**ou **kn**ow **h**ow **y**ou **c**an **te**ll **w**ho ",
            "**t**he **ar**ch-**vil**lain'**s** **go**ing **t**o **b**e? ",
            "**H**e'**s** **t**he **ex**act **oppo**site **o**f **t**he **he**ro. ",
            "**A**nd **mo**st **ti**mes **th**ey'**r**e **fri**ends, **li**ke ",
            "**y**ou **a**nd **m**e! **I** **sho**uld'**v**e **kn**own **w**ay ",
            "**ba**ck **wh**en... **Y**ou **kn**ow **w**hy, **Da**vid? ",
            "**Bec**ause **o**f **t**he **ki**ds. **Th**ey **cal**led **m**e ",
            "**M**r **Gl**ass."
        );
        assert_eq!(expected, result.unwrap());
    }
}
