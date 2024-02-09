use std::io::{Error, Read, Write};

use flate2::write::ZlibEncoder;
use flate2::Compression;

pub(super) fn encode(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(bytes)?;
    Ok(encoder.finish()?)
}

pub(super) fn decode(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decoder = flate2::read::ZlibDecoder::new(bytes);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = [0; 128];
        let expected = [
            120, 218, 197, 192, 129, 0, 0, 0, 0, 128, 160, 252, 169, 199, 56, 94, 1, 0, 128, 0, 1,
        ];
        assert_eq!(encode(&input).unwrap(), expected);
    }

    #[test]
    fn test_decode() {
        let input = [
            120, 218, 197, 192, 129, 0, 0, 0, 0, 128, 160, 252, 169, 199, 56, 94, 1, 0, 128, 0, 1,
        ];
        let expected = [0; 128];
        assert_eq!(decode(&input).unwrap(), expected);
    }
}
