use crate::{doc::*, token::Tokenizer};
use memmap::Mmap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

#[derive(PartialEq, Eq, Debug)]
enum Mode {
    And,
    Or,
    Single,
}
struct Request<'a> {
    pub mode: Mode,
    pub sets: Vec<(&'a [u8], usize)>,
}

impl<'a> Request<'a> {
    fn from_token(token: &'a mut [u8]) -> Request<'a> {
        let mode = match token.split(|x| *x == b' ').nth(1) {
            Some(b"AND") => Mode::And,
            Some(b"OR") => Mode::Or,
            None => Mode::Single,
            _ => {
                println!("Invalid mode");
                Mode::Single
            }
        };
        let mut strategy = BTreeMap::new();
        Tokenizer::new(token)
            .iter()
            .enumerate()
            .filter(|x| (x.0 % 2) == 0)
            .for_each(|(_, x)| {
                *strategy.entry(x).or_default() += 1;
            });
        let sets = strategy.into_iter().map(|(a, b)| (&*a, b)).collect();
        Self { mode, sets }
    }
    fn lookup_single(self, tree: &ArchivedTree) -> Vec<(u32, f64)> {
        tree.tfidf(self.sets[0].0, 1.0).collect()
    }
    fn lookup_or(self, tree: &ArchivedTree) -> Vec<(u32, f64)> {
        let mut tfidfs: Vec<(u32, f64)> = Vec::new();

        for (word, multiplier) in self.sets {
            let mut a_tfidfs = std::mem::take(&mut tfidfs).into_iter().peekable();
            let mut b_tfidfs = tree.tfidf(word, multiplier as f64).peekable();
            loop {
                if a_tfidfs.peek().is_none() {
                    tfidfs.extend(b_tfidfs);
                    break;
                }
                if b_tfidfs.peek().is_none() {
                    tfidfs.extend(a_tfidfs);
                    break;
                }
                if a_tfidfs.peek().unwrap().0 > b_tfidfs.peek().unwrap().0 {
                    tfidfs.push(b_tfidfs.next().unwrap());
                } else {
                    tfidfs.push(a_tfidfs.next().unwrap());
                }
                if tfidfs.len() >= 2
                    && tfidfs.get(tfidfs.len() - 2).map(|(a, _)| a) == tfidfs.last().map(|(a, _)| a)
                {
                    let (_, tfidf) = tfidfs.pop().unwrap();
                    tfidfs.last_mut().unwrap().1 += tfidf;
                }
            }
        }

        tfidfs
    }
    fn lookup_and(self, tree: &ArchivedTree) -> Vec<(u32, f64)> {
        let mut sets = self.sets.into_iter();

        let mut tfidfs: Vec<(u32, f64)> = match sets.next() {
            Some((x, multiplier)) => tree.tfidf(x, multiplier as f64).collect(),
            None => {
                return Vec::new();
            }
        };

        for (word, multiplier) in sets {
            let mut a_tfidfs = std::mem::take(&mut tfidfs).into_iter().peekable();
            let mut b_tfidfs = tree.tfidf(word, multiplier as f64).peekable();
            loop {
                match (a_tfidfs.peek(), b_tfidfs.peek()) {
                    (Some((a_id, a_score)), Some((b_id, b_score))) if a_id == b_id => {
                        tfidfs.push((*a_id, *a_score + *b_score));
                        a_tfidfs.next();
                        b_tfidfs.next();
                    }
                    (Some((a_id, _)), Some((b_id, _))) if a_id > b_id => {
                        b_tfidfs.next();
                    }
                    (Some((a_id, _)), Some((b_id, _))) if a_id < b_id => {
                        a_tfidfs.next();
                    }
                    _ => break,
                }
            }
        }

        tfidfs
    }
    fn lookup_with_max(self, tree: &ArchivedTree, max: usize) -> Vec<isize> {
        let mut result = match self.mode {
            Mode::And => self.lookup_and(tree),
            Mode::Or => self.lookup_or(tree),
            Mode::Single => self.lookup_single(tree),
        };
        // sort
        result.par_sort_unstable_by(|a, b| {
            let a = OrderedFloat(a.1);
            let b = OrderedFloat(b.1);
            b.total_cmp(&a)
        });
        // convert to isize
        let mut result = result
            .into_iter()
            .map(|(a, _)| a as isize)
            .collect::<Vec<_>>();
        result.resize(max, -1);
        result
    }
}

struct InputParser<'a> {
    tree: &'a ArchivedTree,
}

impl<'a> InputParser<'a> {
    fn from_byte(buf: &'a [u8]) -> Self {
        let tree = unsafe { rkyv::archived_root::<Tree>(buf) };
        Self { tree }
    }
    fn parse_input(self, input: impl Read) -> Searcher<'a> {
        let mut input = BufReader::new(input);
        let mut n = String::new();
        input.read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();

        let mut buf = Vec::new();
        input.read_to_end(&mut buf).unwrap();
        Searcher {
            max: n,
            buf,
            tree: self.tree,
        }
    }
}
struct Searcher<'a> {
    max: usize,
    tree: &'a ArchivedTree,
    buf: Vec<u8>,
}

impl Searcher<'_> {
    fn search(mut self) -> Vec<u8> {
        self.buf
            .par_split_mut(|&x| x == b'\n')
            .filter(|x| !x.is_empty())
            .map(move |x| {
                let req = Request::from_token(x);
                req.lookup_with_max(self.tree, self.max)
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes()
    }
}

pub fn search(index: impl AsRef<Path>, input: impl AsRef<Path>) {
    let index = File::open(index).unwrap();
    let mmap = unsafe { Mmap::map(&index) }.unwrap();
    debug_assert_eq!(0, ((*mmap).as_ptr() as usize) % 8);

    let output = InputParser::from_byte(&mmap)
        .parse_input(File::open(input).unwrap())
        .search();

    let mut output_file = File::create("output.txt").unwrap();
    output_file.write_all(&output).unwrap();
    output_file.flush().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_request() {
        let mut token = b"aa AND aaa OR aaaa AND aaaa".to_vec();
        let req = Request::from_token(&mut token);
        assert_eq!(req.mode, Mode::And);
        let mut result = req.sets.into_iter();
        assert_eq!(result.next().unwrap(), (b"aa".as_ref(), 1));
        assert_eq!(result.next().unwrap(), (b"aaa".as_ref(), 1));
        assert_eq!(result.next().unwrap(), (b"aaaa".as_ref(), 2));
    }
}
