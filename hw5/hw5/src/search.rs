use crate::{doc::*, token::Tokenizer};
use hashbrown::HashMap;
use memmap::Mmap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

const EFFECTIVE_OPTIMIZE_RATIO: usize = 3;
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
        tree.tfidf(self.sets[0].0)
    }
    fn lookup_or(self, tree: &ArchivedTree) -> Vec<(u32, f64)> {
        let mut tfidfs: HashMap<u32, f64> = HashMap::new();

        for (word, multiplier) in self.sets {
            for (doc, tfidf) in tree.tfidf(word) {
                let val = tfidfs.entry(doc).or_insert_with(|| 0.0);
                *val += tfidf * multiplier as f64;
            }
        }

        tfidfs.into_iter().collect()
    }
    fn lookup_and(self, tree: &ArchivedTree) -> Vec<(u32, f64)> {
        let mut tfidfs: HashMap<u32, (f64, usize)> = HashMap::new();

        let required_occurance = self.sets.len();
        for (word, multiplier) in self.sets {
            for (doc, tfidf) in tree.tfidf(word) {
                let val = tfidfs.entry(doc).or_insert_with(|| (0.0, 0));
                val.0 += tfidf * multiplier as f64;
                val.1 += 1;
            }
        }

        tfidfs
            .into_par_iter()
            .filter(|(_, v)| v.1 >= required_occurance)
            .map(|(a, (b, _))| (a, b))
            .collect()
    }
    fn lookup_with_max(self, tree: &ArchivedTree, max: usize) -> Vec<isize> {
        fn find_n_largest(
            iter: impl ParallelIterator<Item = (u32, OrderedFloat<f64>)>,
            n: usize,
        ) -> Vec<u32> {
            iter.fold(
                || BinaryHeap::with_capacity(n),
                |mut acc, x| {
                    if acc.len() < n {
                        acc.push(x);
                    } else if acc.peek().unwrap().1 > x.1 {
                        *acc.peek_mut().unwrap() = x;
                    }
                    acc
                },
            )
            .reduce(
                || BinaryHeap::new(),
                |mut h1, h2| {
                    for x in h2 {
                        if h1.len() < n {
                            h1.push(x);
                        } else if *h1.peek().unwrap() > x {
                            *h1.peek_mut().unwrap() = x;
                        }
                    }
                    h1
                },
            )
            .into_iter()
            .map(|x| x.0)
            .collect()
        }
        let mut result = match self.mode {
            Mode::And => self.lookup_and(tree),
            Mode::Or => self.lookup_or(tree),
            Mode::Single => self.lookup_single(tree),
        };
        if result.len() >= max * EFFECTIVE_OPTIMIZE_RATIO {
            let largest_n = find_n_largest(
                result
                    .into_iter()
                    .map(|(a, b)| (a, b.into()))
                    .collect::<Vec<_>>()
                    .into_par_iter(),
                max,
            );
            let result = largest_n
                .into_iter()
                .map(|x| x as isize)
                .collect::<Vec<_>>();
            result
        } else {
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
            return result;
        }
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
                req.lookup_with_max(&self.tree, self.max)
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
