use super::END_TEXT;

pub(super) fn is_end_text(byte: &u8) -> bool {
    byte == &END_TEXT[0]
}

pub(super) trait AdvanceIndex {
    fn advance_by(&mut self, n: usize);
}

impl AdvanceIndex for usize {
    fn advance_by(&mut self, n: usize) {
        *self += n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_end_text() {
        assert_eq!(is_end_text(&0x03), true);
        assert_eq!(is_end_text(&0x04), false);
    }

    #[test]
    fn test_advance_index() {
        let mut index = 0;
        index.advance_by(5);
        assert_eq!(index, 5);
    }
}
