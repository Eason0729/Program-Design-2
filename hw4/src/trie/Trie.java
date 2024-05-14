package trie;

public class Trie {
    long rustInstance;

    public Trie() {
        rustInstance = TrieSys.treeInit();
    }

    protected Trie(long rustInstance) {
        this.rustInstance = rustInstance;
    }

    /**
     * Insert value on path
     *
     * @param path
     * @param value
     */
    public void insert(String path, int value) {
        TrieSys.treeInsert(rustInstance, path, value);
    }

    /**
     * Increase value by one on path
     *
     * @param path
     */
    public void increase(String path) {
        TrieSys.treeIncrease(rustInstance, path);
    }

    /**
     * Get value by path
     *
     * @param path
     * @return value in map
     */
    public int get(String path) {
        return TrieSys.treeGet(rustInstance, path);
    }

    public void merge(Trie other) {
        TrieSys.treeMerge(rustInstance, other.rustInstance);
    }

    @Override
    protected Object clone() throws CloneNotSupportedException {
        long newRustInstance = TrieSys.treeClone(rustInstance);
        return new Trie(newRustInstance);
    }

    @Override
    protected void finalize() throws Throwable {
        TrieSys.treeDrop(rustInstance);
        super.finalize();
    }
}
