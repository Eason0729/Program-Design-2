use rayon::prelude::*;

/// convert a raw byte to a variant
///
/// return true if it is a variant, false otherwise
#[inline]
fn make_variant_in_place(raw: &mut u8) -> bool {
    match *raw {
        b'a'..=b'z' => true,
        b'A'..=b'Z' => {
            *raw += 32;
            true
        }
        _ => false,
    }
}
#[inline]
fn is_variant(raw: u8) -> bool {
    match raw {
        b'a'..=b'z' => true,
        b'A'..=b'Z' => true,
        _ => false,
    }
}
pub struct Tokenizer<'a> {
    raw: &'a mut [u8],
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Tokenizer {
            raw: slice,
            cursor: 0,
        }
    }
    pub fn iter(self) -> impl Iterator<Item = &'a mut [u8]> {
        self.raw.split_mut(|&x| !is_variant(x)).map(|x| {
            x.iter_mut().for_each(|x| {
                make_variant_in_place(x);
            });
            x
        })
    }
    pub fn next(&mut self) -> Option<&[u8]> {
        let mut start = self.cursor;
        loop {
            match self.raw.get_mut(self.cursor) {
                Some(current) => {
                    self.cursor += 1;
                    if !make_variant_in_place(current) {
                        if (start + 1) != self.cursor {
                            break Some(&self.raw[start..(self.cursor - 1)]);
                        }
                        start = self.cursor;
                    }
                }
                None => {
                    if start != self.cursor {
                        break Some(&self.raw[start..self.cursor]);
                    }
                    break None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let mut input = b"Hello, world!".to_vec();
        let mut tokenizer = Tokenizer::new(&mut input);
        assert_eq!(tokenizer.next().unwrap(), b"hello");
        assert_eq!(tokenizer.next().unwrap(), b"world");
        assert!(tokenizer.next().is_none());
    }
}
