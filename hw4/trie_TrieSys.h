/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class trie_TrieSys */

#ifndef _Included_trie_TrieSys
#define _Included_trie_TrieSys
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     trie_TrieSys
 * Method:    treeInit
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_trie_TrieSys_treeInit
  (JNIEnv *, jclass);

/*
 * Class:     trie_TrieSys
 * Method:    treeInsert
 * Signature: (JLjava/lang/String;I)V
 */
JNIEXPORT void JNICALL Java_trie_TrieSys_treeInsert
  (JNIEnv *, jclass, jlong, jstring, jint);

/*
 * Class:     trie_TrieSys
 * Method:    treeIncrease
 * Signature: (JLjava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_trie_TrieSys_treeIncrease
  (JNIEnv *, jclass, jlong, jstring);

/*
 * Class:     trie_TrieSys
 * Method:    treeGet
 * Signature: (JLjava/lang/String;)I
 */
JNIEXPORT jint JNICALL Java_trie_TrieSys_treeGet
  (JNIEnv *, jclass, jlong, jstring);

/*
 * Class:     trie_TrieSys
 * Method:    treeDrop
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_trie_TrieSys_treeDrop
  (JNIEnv *, jclass, jlong);

/*
 * Class:     trie_TrieSys
 * Method:    treeMerge
 * Signature: (JJ)V
 */
JNIEXPORT void JNICALL Java_trie_TrieSys_treeMerge
  (JNIEnv *, jclass, jlong, jlong);

/*
 * Class:     trie_TrieSys
 * Method:    treeClone
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_trie_TrieSys_treeClone
  (JNIEnv *, jclass, jlong);

/*
 * Class:     trie_TrieSys
 * Method:    treeBulkIncrease
 * Signature: (J[Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_trie_TrieSys_treeBulkIncrease
  (JNIEnv *, jclass, jlong, jobjectArray);

/*
 * Class:     trie_TrieSys
 * Method:    treeBulkIncreaseTokenized
 * Signature: (JLjava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_trie_TrieSys_treeBulkIncreaseTokenized
  (JNIEnv *, jclass, jlong, jstring);

#ifdef __cplusplus
}
#endif
#endif