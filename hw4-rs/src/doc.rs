use std::{
    mem::size_of,
    ops::{Deref, DerefMut},
    slice,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use rayon::prelude::*;
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    task::{spawn_blocking, JoinSet},
};

use crate::{doc, token::Tokenizer, tree::Tree};

const DOC_LINE: usize = 5;

/// merge a slice of mutable slices into a single mutable slice
///
/// # Safety
///
/// The caller must ensure that the slices are continous and non-overlapping
unsafe fn merge_slice_mut<'a, T: Sized>(a: &[&'a mut [T]]) -> &'a mut [T] {
    let last_len = a.last().unwrap().len();
    let start_ptr = a.first().unwrap().deref().as_ptr() as *mut T;
    let end_ptr = a.last().unwrap().deref().as_ptr();

    let len = (last_len + end_ptr as usize - start_ptr as usize) / size_of::<T>();
    slice::from_raw_parts_mut(start_ptr, len)
}
pub struct Documents {
    docs: Vec<Document>,
    occurance: Tree,
}

impl Documents {
    pub async fn from_reader(mut reader: impl AsyncRead + Unpin) -> Self {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await.unwrap();

        let buffer = buffer.leak();
        let buffer_ptr = buffer.as_mut_ptr();

        let occurance = Tree::new();
        let mut set = JoinSet::new();

        {
            let occurance = unsafe { &*(&occurance as *const Tree) };
            buffer
                .split_mut(|&x| x == b'\n')
                .collect::<Vec<_>>()
                .chunks(5)
                .map(|x| unsafe { merge_slice_mut(x) })
                .for_each(|x| {
                    set.spawn_blocking(move || {
                        let doc = Document::from_buffer(x);
                        unsafe { occurance.merge(&doc.words) };
                        doc
                    });
                });
        }

        let mut docs = Vec::new();
        while let Some(x) = set.join_next().await {
            docs.push(x.unwrap());
        }

        drop(unsafe { Box::from_raw(buffer_ptr) });

        Documents { docs, occurance }
    }
    pub fn get_document(&self, idx: usize) -> Option<&Document> {
        self.docs.get(idx)
    }
    pub fn inverse_document_frequency(&self, token: &[u8]) -> f64 {
        f64::log10(self.docs.len() as f64 / self.occurance.get(token).unwrap_or_default() as f64)
    }
}

pub struct Document {
    words: Tree,
    word_count: usize,
}

impl Document {
    fn insert(&self, token: &[u8]) {
        self.words.insert(token);
    }
    pub fn from_buffer(mut buffer: &mut [u8]) -> Self {
        let mut word_count = 0;
        let tree = Tree::new();

        let mut tokenizer = Tokenizer::new(&mut buffer);
        // tokenizer.par_iter().for_each(|x| {
        //     word_count.fetch_add(1, Ordering::AcqRel);
        //     tree.insert(x);
        // });

        while let Some(token) = tokenizer.next() {
            word_count += 1;
            tree.insert(token);
        }
        Document {
            words: tree,
            word_count,
        }
    }
    pub fn term_frequency(&self, token: &[u8]) -> f64 {
        if self.word_count == 0 {
            return 0.0;
        }
        let count = self.words.get(token).unwrap_or_default();
        count as f64 / self.word_count as f64
    }
}
