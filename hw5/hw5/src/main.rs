mod doc;
mod index;
mod search;
mod string;
mod token;

fn main() {
    index::index("test/corpus0.txt", "corpus0.ser");
    search::search("test/corpus0.txt", "test/tc0.txt");
}
