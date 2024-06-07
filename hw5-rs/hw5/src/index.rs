use rayon::iter::IndexedParallelIterator;
use rayon::prelude::*;
use rkyv::{
    ser::{serializers::*, Serializer},
    AlignedVec,
};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    mem::size_of,
    ops::Deref,
    path::Path,
};

use crate::{doc::*, token::Tokenizer};

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
    std::slice::from_raw_parts_mut(start_ptr, len)
}

pub fn index(input: impl AsRef<Path>, output: impl AsRef<Path>) -> AlignedVec {
    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)
        .unwrap();
    let mut input = File::open(input).unwrap();
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).unwrap();

    let mut tree = Tree::default();

    for (id, doc) in buf
        .split_mut(|x| *x == b'\n')
        .collect::<Vec<_>>()
        .par_chunks(5)
        .map(|x| {
            let buf = unsafe { merge_slice_mut(x) };
            let mut tokenizer = Tokenizer::new(buf);
            let mut doc = Document::default();
            while let Some(x) = tokenizer.next() {
                doc.add(x);
            }
            doc
        })
        .enumerate()
        .collect::<Vec<_>>()
    {
        tree.insert_doc(id as u64, doc)
    }

    let mut serializer = AllocSerializer::<0>::default();
    serializer.serialize_value(&tree).unwrap();
    let bytes = serializer.into_serializer().into_inner();
    output.write_all(&bytes).unwrap();
    output.flush().unwrap();
    bytes
}
