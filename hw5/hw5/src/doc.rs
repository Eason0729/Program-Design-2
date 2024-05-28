use std::collections::BTreeMap;

use rkyv::{bytecheck, rend::LittleEndian, Archive, CheckBytes, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, CheckBytes))]
pub struct Documents {
    /// how many word is in the Document(by idx)
    word_counts: BTreeMap<u64, u64>,
}

impl Documents {
    fn insert(&mut self, id: u64, word_count: u64) {
        debug_assert!(word_count > 0);
        self.word_counts.insert(id, word_count);
    }
}

impl ArchivedDocuments {
    fn term_frequency(&self, word_counts: &[LittleEndian<u64>]) -> BTreeMap<u64, f64> {
        let mut term_frequency = BTreeMap::new();
        for (doc, count) in &self.word_counts {
            let doc: u64 = doc.into();
            let count: u64 = count.into();
            term_frequency.insert(
                doc,
                count as f64 / Into::<u64>::into(word_counts[doc as usize]) as f64,
            );
        }
        term_frequency
    }
    fn inverse_document_frequency(&self, total_docs: u64) -> f64 {
        f64::log2(total_docs as f64 / self.word_counts.len() as f64)
    }
}

#[derive(Default)]
pub struct Document {
    word_counts: BTreeMap<Vec<u8>, u64>,
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

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug, CheckBytes))]
pub struct Tree {
    tree: BTreeMap<Vec<u8>, Documents>,
    total_docs: u64,
    word_counts: Vec<u64>,
}

impl Tree {
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

impl ArchivedTree {
    fn get(&self, word: &[u8]) -> Option<&ArchivedDocuments> {
        self.tree.get(word)
    }
    pub fn TFIDF(&self, word: &[u8]) -> BTreeMap<u64, f64> {
        let total_docs: u64 = self.total_docs.into();
        let mut tfidf = BTreeMap::new();
        if let Some(docs) = self.get(word) {
            let word_counts = &self.word_counts;
            let term_frequency = docs.term_frequency(word_counts);
            let inverse_document_frequency = docs.inverse_document_frequency(total_docs);
            for (doc, term_frequency) in term_frequency {
                tfidf.insert(doc, term_frequency * inverse_document_frequency);
            }
        }
        tfidf
    }
}

// #[cfg(test)]
// mod test {
//     use rkyv::{rancor::Error, to_bytes};

//     use super::*;

//     #[test]
//     fn test_documents() {
//             let word_counts=vec![1,2,3,5,6];
//             let docs= Documents {
//             word_counts: vec![(1, 2), (2, 3), (10,4)].into_iter().collect(),
//         };
//         let docs=to_bytes::<Error>(&docs).unwrap();

//     }
// }
