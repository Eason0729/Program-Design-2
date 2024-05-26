mod tree;
use jni::objects::*;
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use tree::{Tree, TreeWrapper};

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeInit<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
) -> jlong {
    let tree: Tree<usize> = Tree::new();
    let ptr = tree.into_ptr();
    i64::from_ne_bytes(ptr.to_ne_bytes())
}

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeInsert<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    input: JString<'local>,
    value: jint,
) {
    let input: String = env.get_string(&input).unwrap().into();
    let mut tree =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
    tree.insert(input.chars(), value as usize);
}

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeIncrease<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    input: JString<'local>,
) {
    let input: String = env.get_string(&input).unwrap().into();
    let mut tree =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
    *tree.get_mut(input.chars()) += 1;
}

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeGet<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    input: JString<'local>,
) -> jint {
    let input: String = env.get_string(&input).unwrap().into();
    let mut tree =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
    tree.get(input.chars()).cloned().unwrap_or_default() as jint
}

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeDrop<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) {
    let tree = unsafe { Tree::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
    drop(tree);
}

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeMerge<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr1: jlong,
    ptr2: jlong,
) {
    let mut tree1 =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr1.to_ne_bytes()) as usize) };
    let tree2 =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr2.to_ne_bytes()) as usize) };
    tree1.increase_if_exist(&tree2);
}

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeClone<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
) -> jlong {
    let tree =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
    let cloned = tree.clone();
    i64::from_ne_bytes(cloned.into_ptr().to_ne_bytes())
}

// #[no_mangle]
// pub extern "system" fn Java_trie_TrieSys_treeBulkIncrease<'local>(
//     mut env: JNIEnv<'local>,
//     class: JClass<'local>,
//     ptr: jlong,
//     paths:JObjectArray<'local>,
// ){
//     let mut tree =
//         unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
//     let len=env.get_array_length(&paths).unwrap();

//     for i in 0..len{
//         let path=env.get_object_array_element(&paths,i).unwrap();
//         let path: String = env.get_string(&path.into()).unwrap().into();
//         *tree.get_mut(path.chars())+=1;
//     }
// }

#[no_mangle]
pub extern "system" fn Java_trie_TrieSys_treeBulkIncreaseTokenized<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    ptr: jlong,
    tokens: JString<'local>,
){
    let raw_tokens: String = env.get_string(&tokens).unwrap().into();
    let tokens=raw_tokens.split_whitespace();
    let mut tree =
        unsafe { TreeWrapper::<usize>::from_ptr(i64::from_ne_bytes(ptr.to_ne_bytes()) as usize) };
    for token in tokens{
        *tree.get_mut(token.chars())+=1;
    }
}

