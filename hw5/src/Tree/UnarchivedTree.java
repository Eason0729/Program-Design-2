package Tree;

import java.io.File;
import java.nio.file.Path;
import java.util.ArrayList;

public class UnarchivedTree<V extends Bincode>{
    private int len;
    private final ArrayList<Node<V>> nodes;
    public UnarchivedTree() {
        this.len = 0;
        this.nodes = new ArrayList<>();
        nodes.add(new Node<>());
    }
    private Node<V> get_root() {
        return nodes.get(0);
    }
    private int nodeGetOrInsert(byte c, int index) {
        Integer id = get_root().getChild(c);
        if(id==null) {
            nodes.add(new Node<>());
            get_root().setChild(c, len);
            len++;
            return len-1;
        }
        return id;
    }
    private Integer nodeGet(byte c, int index) {
        Integer id = get_root().getChild(c);
        if(id==null) return null;
        return id;
    }
    public void insert(String s, V value) {
        Node<V> cur = get_root();
        for(int i=0; i<s.length(); i++) {
            byte c = (byte)s.charAt(i);
            int id = nodeGetOrInsert(c, 0);
            cur = nodes.get(id);
        }
        cur.value = value;
    }
    public V get(String s) {
        Node<V> cur = get_root();
        for(int i=0; i<s.length(); i++) {
            byte c = (byte)s.charAt(i);
            Integer id = nodeGet(c, 0);
            if(id==null) return null;
            cur = nodes.get(id);
        }
        return cur.value;
    }
    public void archive(Path path) {
        File file = path.toFile();
    }
}
