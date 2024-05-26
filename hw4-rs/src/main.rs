use std::env;
use tokio::fs::{self, File};

use doc::Documents;
use rayon::prelude::*;

mod doc;
mod token;
mod tree;

#[cfg(feature = "release")]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let doc_source = args.get(1).expect("No document source provided");
    let testcase = args.get(2).expect("No testcase provided");

    let file = File::open(doc_source).await.unwrap();
    let docs = Documents::from_reader(file).await;

    let testcase = fs::read_to_string(testcase).await.unwrap();
    let mut testcase = testcase.lines();

    let terms = testcase
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    let doc_secs = testcase
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let results = terms
        .par_iter()
        .zip(doc_secs.par_iter())
        .map(|(term, sec)| {
            let idf = docs.inverse_document_frequency(term.as_bytes());
            let tf = docs
                .get_document(*sec)
                .unwrap()
                .term_frequency(term.as_bytes());
            format!("{:.5} ", tf * idf)
        })
        .collect::<Vec<_>>()
        .concat();

    fs::write("output.txt", results.as_bytes()).await.unwrap();
}
