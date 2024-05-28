use memmap::Mmap;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    ops::Deref,
    path::Path,
};

use crate::{
    doc::{ArchivedTree, Tree},
    token::Tokenizer,
};
use rkyv::validation::validators::check_archived_root;

enum Mode {
    And,
    Or,
    Single,
}
struct Request<'a> {
    mode: Mode,
    sets: Vec<(&'a [u8], usize)>,
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
        let tokenizer = Tokenizer::new(token);
        let mut strategy = BTreeMap::new();
        tokenizer.iter().for_each(|x| {
            *strategy.entry(x).or_default() += 1;
        });
        let sets = strategy.into_iter().map(|(a, b)| (&*a, b)).collect();
        Self { mode, sets }
    }
    fn lookup(self, tree: &ArchivedTree, max: usize) -> Vec<isize> {
        let mut tfidfs: BTreeMap<u32, f64> = BTreeMap::new();
        for (word, multiplier) in self.sets {
            for (doc, tfidf) in tree.TFIDF(word) {
                println!("{} {}", doc, tfidf);
                *tfidfs.entry(doc).or_default() += tfidf * multiplier as f64;
            }
        }
        let mut result = Vec::new();
        let mut tfidfs = tfidfs.iter();
        while result.len() < max {
            match tfidfs.next_back() {
                Some((doc, _)) => result.push(*doc as isize),
                None => break,
            }
        }
        result.resize(max, -1);
        result
    }
}

struct InputParser<'a> {
    tree: &'a ArchivedTree,
}

impl<'a> InputParser<'a> {
    fn from_mmap(mmap: &'a Mmap) -> Self {
        let buf = mmap.deref();
        let tree = unsafe { rkyv::check_archived_root::<Tree>(buf) }.unwrap();
        Self { tree }
    }
    fn parse_input(self, input: impl AsRef<Path>) -> Searcher<'a> {
        println!("{:?}", input.as_ref());
        let mut input = BufReader::new(File::open(input).unwrap());
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

    let output = InputParser::from_mmap(&mmap).parse_input(input).search();

    let mut output_file = File::create("output.txt").unwrap();
    output_file.write_all(&output).unwrap();
    output_file.flush().unwrap();
}
