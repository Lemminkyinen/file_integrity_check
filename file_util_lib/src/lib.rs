mod compress;
mod misc;

use compress::{decode, encode};
use misc::{is_end_text, AdvanceIndex};
use pyo3::prelude::*;
use std::io::{Read, Write};

const END_TEXT: [u8; 1] = [0x03];
const HASH_SIZE: usize = 32;

#[pyfunction]
fn save_hashes(hashes: Vec<(String, [u8; HASH_SIZE])>) -> PyResult<()> {
    let mut file = std::fs::File::create("hashes")?;

    let buffer: Vec<u8> = hashes
        .into_iter()
        .flat_map(|(name, hash)| {
            let mut concatenated: Vec<u8> = Vec::new();
            concatenated.extend_from_slice(&hash);
            concatenated.extend_from_slice(name.as_bytes());
            concatenated.extend_from_slice(&END_TEXT);
            concatenated
        })
        .collect();

    let encoded = encode(&buffer)?;
    file.write_all(&encoded)?;
    Ok(())
}

#[pyfunction]
fn read_hashes() -> PyResult<Vec<(String, [u8; HASH_SIZE])>> {
    let mut hashes = Vec::new();
    let Ok(mut file) = std::fs::File::open("hashes") else {
        println!("hashes file not found");
        return Ok(hashes);
    };

    let mut buffer = Vec::new();
    file.read_to_end(buffer.as_mut())?;
    let buffer = decode(&buffer)?;

    let mut index = 0;
    loop {
        let Some(hash_buff) = buffer.get(index..index + HASH_SIZE) else {
            // this is the expected break condition
            break;
        };
        index.advance_by(HASH_SIZE);
        let Some(separator_index) = buffer[index..].iter().position(is_end_text) else {
            println!("separator not found");
            break;
        };
        let Some(name_buff) = buffer.get(index..index + separator_index) else {
            println!("name not found");
            break;
        };
        index.advance_by(separator_index + 1);
        let name = String::from_utf8(name_buff.to_vec())?;
        let hash = hash_buff.try_into()?;
        hashes.push((name, hash));
    }
    return Ok(hashes);
}

/// A Python module implemented in Rust.
#[pymodule]
fn file_util_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(save_hashes, m)?)?;
    m.add_function(wrap_pyfunction!(read_hashes, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_hashes() {
        let hashes = vec![
            (
                "file1".to_string(),
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
                    0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
                ],
            ),
            (
                "file2".to_string(),
                [
                    0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
                    0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a,
                    0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40,
                ],
            ),
        ];
        save_hashes(hashes.clone()).expect("save failed");
        let read_hashes = read_hashes().expect("read failed");
        assert_eq!(hashes, read_hashes);
    }

    #[test]
    fn test_read_hashes() {
        let hashes = vec![
            (
                "file1".to_string(),
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                    0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
                    0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
                ],
            ),
            (
                "file2".to_string(),
                [
                    0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
                    0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a,
                    0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40,
                ],
            ),
        ];
        save_hashes(hashes.clone()).expect("save failed");
        let read_hashes = read_hashes().expect("read failed");
        assert_eq!(hashes, read_hashes);
    }
}
