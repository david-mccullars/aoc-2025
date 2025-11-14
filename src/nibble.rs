use std::iter::Peekable;

pub trait NibbleIter {
    fn nibbles(&self) -> Nibbles<'_>;
}

impl NibbleIter for [u8] {
    fn nibbles(&self) -> Nibbles<'_> {
        Nibbles {
            bytes: self,
            index: 0,
        }
    }
}

pub struct Nibbles<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> Iterator for Nibbles<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_idx = self.index / 2;
        if byte_idx >= self.bytes.len() {
            return None;
        }
        let nibble = if self.index % 2 == 0 {
            self.bytes[byte_idx] / 16
        } else {
            self.bytes[byte_idx] % 16
        };
        self.index += 1;
        Some(nibble)
    }
}

pub trait RepeatedRuns: Iterator<Item = u8> + Sized {
    fn repeated(self, n: usize) -> Repeated<Self>;
}

impl<I: Iterator<Item = u8>> RepeatedRuns for I {
    fn repeated(self, n: usize) -> Repeated<Self> {
        Repeated {
            iter: self.peekable(),
            min_length: n,
        }
    }
}

pub struct Repeated<I: Iterator<Item = u8>> {
    iter: Peekable<I>,
    min_length: usize,
}

impl<I: Iterator<Item = u8>> Iterator for Repeated<I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(val) = self.iter.next() {
            let mut count = 1;
            while self.iter.peek() == Some(&val) {
                self.iter.next();
                count += 1;
            }
            if count >= self.min_length {
                return Some(val);
            }
        }
        None
    }
}
