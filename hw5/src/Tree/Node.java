package Tree;

public class Node <V extends Bincode>{
    public V value;
    private final int[] children;
    public Node() {
        this.value=null;
        this.children = new int[26];
    }
    public Node(V value) {
        this.value = value;
        this.children = new int[26];
    }
    public void setChild(byte c, int index) {
        children[c-'a'] = index+1;
    }
    public Integer getChild(byte c) {
        int id= children[c-'a'];

        if(id==0) return null;
        return id-1;
    }
}
