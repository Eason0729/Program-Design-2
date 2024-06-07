use memmap::Mmap;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use crate::{doc::*, token::Tokenizer};
const DEVIATION: f64 = 1e-20;

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
    fn lookup(self, tree: &ArchivedTree, max: usize) -> Vec<isize> {
        let mut tfidfs: BTreeMap<u64, (f64, usize)> = BTreeMap::new();

        let required_occurance = match self.mode {
            Mode::And => self.sets.len(),
            Mode::Or => 0,
            Mode::Single => 0,
        };
        for (word, multiplier) in self.sets {
            for (doc, tfidf) in tree.tfidf(word) {
                let val = tfidfs.entry(doc).or_insert_with(|| (0.0, 0));
                val.0 += tfidf * multiplier as f64;
                val.1 += 1;
            }
        }
        let mut result = Vec::new();

        let mut tfidfs = tfidfs
            .into_iter()
            .rev()
            .filter(|(_, v)| v.1 >= required_occurance)
            .collect::<Vec<_>>();
        tfidfs.sort_by(|a, b| {
            if (b.1 .0 - a.1 .0).abs() < DEVIATION {
                return a.0.cmp(&b.0);
            }
            b.1 .0
                .partial_cmp(&a.1 .0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (id, _) in tfidfs {
            result.push(id as isize);
        }

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

impl<'a> Searcher<'a> {
    fn search(mut self) -> Vec<u8> {
        self.buf
            .par_split_mut(|&x| x == b'\n')
            .filter(|x| !x.is_empty())
            .map(move |x| {
                let req = Request::from_token(x);
                req.lookup(&self.tree, self.max)
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
