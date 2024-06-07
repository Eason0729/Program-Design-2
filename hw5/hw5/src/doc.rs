use hashbrown::HashMap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use rkyv::{Archive, Deserialize, Serialize};
use std::collections::BTreeMap;

pub fn e_log(inp: f64) -> f64 {
    unsafe { fdlibm_rs::__ieee754_log(inp.into()) }
}

#[derive(Default)]
pub struct Documents {
    /// how many word is in the Document(by idx)
    word_counts: HashMap<u64, u64>,
}

impl Documents {
    fn insert(&mut self, id: u64, word_count: u64) {
        debug_assert!(word_count > 0);
        self.word_counts.insert(id, word_count);
    }
    fn term_frequency<'a>(
        &'a self,
        word_counts: &'a [u64],
    ) -> impl Iterator<Item = (u64, f64)> + 'a {
        self.word_counts.iter().map(move |(doc, count)| {
            (
                *doc,
                *count as f64 / Into::<u64>::into(word_counts[*doc as usize]) as f64,
            )
        })
    }
    fn inverse_document_frequency(&self, total_docs: u64) -> f64 {
        match self.word_counts.len() {
            0 => 0.0,
            _ => e_log(total_docs as f64 / self.word_counts.len() as f64),
        }
    }
}

#[derive(Default)]
pub struct Document {
    word_counts: HashMap<Vec<u8>, u64>,
    word_count: u64,
}

impl Document {
    pub fn add(&mut self, token: &[u8]) {
        let value = self.word_counts.entry(token.to_vec()).or_default();
        *value += 1;
        self.word_count += 1;
    }
    pub fn from_tokens(tokens: &[&[u8]]) -> Self {
        let mut self_ = Self::default();

        for token in tokens {
            self_.add(&token);
        }
        self_
    }
}

#[derive(Default)]
pub struct TreeBuilder {
    pub tree: HashMap<Vec<u8>, Documents>,
    total_docs: u64,
    word_counts: Vec<u64>,
}

impl TreeBuilder {
    pub fn insert_doc(&mut self, id: u64, doc: Document) {
        self.total_docs += 1;

        self.word_counts.resize(id as usize + 1, 0);
        self.word_counts[id as usize] = doc.word_count;

        for (word, word_count) in doc.word_counts {
            let docs = self.tree.entry(word).or_default();
            docs.insert(id, word_count);
        }
    }
}

impl TreeBuilder {
    pub fn tfidfs(self) -> impl Iterator<Item = (Vec<u8>, Vec<(u32, f64)>)> {
        let total_docs: u64 = self.total_docs.into();
        self.tree.into_iter().map(move |(word, docs)| {
            let word_counts = &self.word_counts;
            let inverse_document_frequency = docs.inverse_document_frequency(total_docs);
            let mut tfidf: Vec<(u32, f64)> = docs
                .term_frequency(word_counts)
                .into_iter()
                .map(|(doc, term_frequency)| {
                    (
                        doc as u32,
                        term_frequency * inverse_document_frequency.clone(),
                    )
                })
                .collect();
            tfidf.par_sort_by(|a, b| a.0.cmp(&b.0));
            (word, tfidf)
        })
    }
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Tree(BTreeMap<Vec<u8>, Vec<(u32, OrderedFloat<f64>)>>);

impl From<crate::doc::TreeBuilder> for Tree {
    fn from(value: crate::doc::TreeBuilder) -> Self {
        let mut tree = BTreeMap::new();
        for (key, value) in value.tfidfs() {
            tree.insert(key, value.into_iter().map(|(a, b)| (a, b.into())).collect());
        }
        Self(tree)
    }
}

impl ArchivedTree {
    pub fn tfidf(&self, word: &[u8]) -> Vec<(u32, f64)> {
        self.0
            .get(word)
            .map(|x| x.iter().map(|(a, b)| (*a, f64::from(*b))).collect())
            .unwrap_or_default()
    }
}
