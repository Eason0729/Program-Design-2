import Index.Tree;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.ObjectInputStream;

public class TFIDFSearch {
    public static void main(String[] args) throws IOException, ClassNotFoundException {
        FileInputStream fis = new FileInputStream("student.ser");
                ObjectInputStream ois = new ObjectInputStream(fis);
        Tree deserializedTree = (Tree) ois.readObject();
    }
}
