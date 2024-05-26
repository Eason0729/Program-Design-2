package trie;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class TrieTest {

    @Test
    void insert() {
        Trie trie = new Trie();
        trie.insert("hello", 1);
        assertEquals(1, trie.get("hello"));
    }

    @Test
    void increase() {
        Trie trie = new Trie();
        trie.increase("hello");
        assertEquals(1, trie.get("hello"));
    }

    @Test
    void merge() {
        Trie trie1 = new Trie();
        trie1.insert("hello", 1);
        Trie trie2 = new Trie();
        trie2.insert("world", 1);
        trie1.merge(trie2);
        assertEquals(1, trie1.get("hello"));
        assertEquals(1, trie1.get("world"));
    }
}