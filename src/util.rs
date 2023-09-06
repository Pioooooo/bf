use crate::Result;
pub(crate) struct IntoChars {
    s: String,
    offset: usize,
}

impl IntoChars {
    pub(crate) fn new(s: String) -> Self {
        IntoChars { s, offset: 0 }
    }
}

impl Iterator for IntoChars {
    type Item = Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = &self.s[self.offset..];

        match remaining.chars().next() {
            Some(c) => {
                self.offset += c.len_utf8();
                Some(Ok(c))
            }
            None => None,
        }
    }
}
