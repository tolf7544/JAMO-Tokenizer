use pyo3::prelude::*;


// https://pyo3.rs/v0.18.1/module

/*
tokenize
combine
jamo_format
encoding
decoding
vocab
Train < Tokenizer를 Injection하여 진행 >
Tokenizer
DataLoader
*/

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn jamo_tokenizer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
