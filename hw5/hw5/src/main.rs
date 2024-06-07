#![feature(trivial_bounds)]
mod doc;
mod index;
mod search;
mod string;
mod token;

fn main() {
    // index::index("test/corpus1.txt", "output.ser");
    // search::search("output.ser", "test/tc1.txt");
    index::index("../testcase/corpus0.txt", "output.ser");
    search::search("output.ser", "../testcase/tc0.txt");
}
