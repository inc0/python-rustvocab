use std::fs::File;
use std::io::{BufRead, BufReader};
use counter::Counter;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn count_words(sentence_list: Vec<Vec<String>>, vocab_length: usize) -> Vec<(std::string::String, usize)>{
    let mut word_count: Counter<String> = Counter::new();
    for sentence in sentence_list {
        word_count += sentence
    }
    let mut res = Vec::new();
    res = word_count.most_common();
    res.truncate(vocab_length);
    return res
}

#[pymodule]
fn vocab(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count_words))?;
    Ok(())
}
