#![feature(trivial_bounds)]

mod doc;
mod index;
mod search;
mod string;
mod token;

use std::path::Path;
use string::{OsStrExt as _, OsStringExt as _};

use jni::{
    objects::{JClass, JObjectArray},
    JNIEnv,
};

pub use index::index;
pub use search::search;

#[no_mangle]
pub extern "system" fn Java_Indexer_buildIndex<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    array: JObjectArray<'local>,
) {
    let source = env.get_object_array_element(&array, 0).unwrap();
    let source: String = env.get_string(&source.into()).unwrap().into();

    let tree_path = Path::new(&source)
        .file_name()
        .unwrap()
        .remove_extension()
        .to_os_string()
        .add_extension("ser");

    index::index(source, tree_path);
}

#[no_mangle]
pub extern "system" fn Java_Indexer_TFIDFSearch<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    array: JObjectArray<'local>,
) {
    let tree_path = env.get_object_array_element(&array, 0).unwrap();
    let tree_path: String = env.get_string(&tree_path.into()).unwrap().into();
    let tree_path = Path::new(&tree_path)
        .file_name()
        .unwrap()
        .to_os_string()
        .add_extension("ser");

    let input = env.get_object_array_element(&array, 1).unwrap();
    let input: String = env.get_string(&input.into()).unwrap().into();

    search::search(tree_path, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        index::index("../testcase/corpus0.txt", "output.ser");
    }

    #[test]
    fn index_and_search() {
        index::index("../testcase/corpus0.txt", "output.ser");
        search::search("output.ser", "../testcase/tc4.txt");
    }
}
