use pyo3::prelude::*;
use sha2::Digest;
use sha2::Sha256;
use std::{fs::File, io::BufReader, io::Read};

#[pyfunction]
fn file_sha256(file_path: &str) -> PyResult<[u8; 32]> {
    let file = File::open(file_path)?;

    let mut hasher = Sha256::new();
    let mut buf_reader = BufReader::new(file);

    let mut buffer = [0u8; 512];
    loop {
        let n = buf_reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let hash = hasher.finalize();
    let mut ret = [0u8; 32];
    ret.copy_from_slice(&hash.as_slice());
    Ok(ret)
}

/// A Python module implemented in Rust.
#[pymodule]
fn hash_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(file_sha256, m)?)?;
    Ok(())
}
