use jni::{
    objects::{JClass, JObjectArray},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_Indexer_buildIndex<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    array: JObjectArray<'local>,
) {
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern "system" fn Java_Indexer_TFIDFSearch<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    array: JObjectArray<'local>,
) {
    println!("Hello from Rust!");
}
