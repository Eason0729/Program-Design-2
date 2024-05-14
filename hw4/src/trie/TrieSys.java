package trie;

import java.nio.file.Path;
import java.nio.file.Paths;

public class TrieSys {
    static {
        Path libPath = Paths.get("trie/target/release/libtrie.so").toAbsolutePath();
        System.load(libPath.toString());
    }

    protected static native long treeInit();

    protected static native void treeInsert(long ptr, String path, int value);

    protected static native void treeIncrease(long ptr, String path);

    protected static native int treeGet(long ptr, String path);

    protected static native void treeDrop(long ptr);

    protected static native void treeMerge(long mut_ptr, long ptr);

    protected static native long treeClone(long ptr);
}
