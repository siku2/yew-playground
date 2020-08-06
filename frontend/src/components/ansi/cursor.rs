use std::slice::SliceIndex;

#[derive(Clone, Debug)]
pub struct CharCursor<'a> {
    s: &'a str,
    pos: usize,
}
impl<'a> CharCursor<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn get<I: SliceIndex<str>>(&self, i: I) -> Option<&'a I::Output> {
        self.s.get(i)
    }

    pub fn remainder(&self) -> &'a str {
        // SAFETY: pos always points to a char boundary
        unsafe { self.s.get_unchecked(self.pos..) }
    }

    fn peek(&self) -> Option<char> {
        self.remainder().chars().next()
    }

    pub fn peek_char(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    fn read_if(&mut self, f: impl FnOnce(char) -> bool) -> Option<char> {
        match self.peek() {
            Some(c) if f(c) => {
                self.pos += c.len_utf8();
                Some(c)
            }
            _ => None,
        }
    }

    pub fn read(&mut self) -> Option<char> {
        self.read_if(|_| true)
    }

    pub fn read_char(&mut self, expected: char) -> Option<char> {
        self.read_if(|c| c == expected)
    }

    pub fn read_while(&mut self, mut f: impl FnMut(char) -> bool) {
        loop {
            if self.read_if(&mut f).is_none() {
                break;
            }
        }
    }
}
