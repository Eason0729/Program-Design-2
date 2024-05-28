use std::collections::BTreeMap;

use rkyv::{Archive, CheckBytes, Deserialize, Serialize,bytecheck};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default,CheckBytes)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Documents {
    /// how many time is occur in Documents
    occurance: u32,
    /// how many word is in the Document(by idx)
    word_counts: BTreeMap<u32, u32>,
}

impl Documents {
    fn insert(&mut self, id: u32, word_count: u32) {
        debug_assert!(word_count > 0);
        self.occurance += 1;
        self.word_counts.insert(id, word_count);
    }
}

impl ArchivedDocuments {
    fn term_frequency(&self, word_counts: &[u32]) -> BTreeMap<u32, f64> {
        let mut term_frequency = BTreeMap::new();
        for (doc, count) in &self.word_counts {
            term_frequency.insert(*doc, *count as f64 / word_counts[*doc as usize] as f64);
        }
        term_frequency
    }
    fn inverse_document_frequency(&self, total_docs: u32) -> f64 {
        f64::log2(self.occurance as f64 / total_docs as f64)
    }
}

#[derive(Default)]
pub struct Document {
    word_counts: BTreeMap<Vec<u8>, u32>,
    word_count: u32,
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

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Default,CheckBytes)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
pub struct Tree {
    tree: BTreeMap<Vec<u8>, Documents>,
    total_docs: u32,
    word_counts: Vec<u32>,
}

impl Tree {
    pub fn insert_doc(&mut self, id: u32, doc: Document) {
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
    pub fn TFIDF(&self, word: &[u8]) -> BTreeMap<u32, f64> {
        let mut tfidf = BTreeMap::new();
        if let Some(docs) = self.get(word) {
            let term_frequency = docs.term_frequency(&self.word_counts);
            let inverse_document_frequency = docs.inverse_document_frequency(self.total_docs);
            for (doc, term_frequency) in term_frequency {
                tfidf.insert(doc, term_frequency * inverse_document_frequency);
            }
        }
        tfidf
    }
}
