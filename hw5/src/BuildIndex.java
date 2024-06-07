import Index.Document;
import Index.Tree;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class BuildIndex {
    BufferedReader input=null;
    File output=null;
    Tree tree = new Tree();
    void parseCliInput(String[]args) throws IOException {
        Path source=Path.of(args[0]);
        String destination=source.getFileName().toString()+".ser";

        input = new BufferedReader(new FileReader(String.valueOf(source)));
        output = new File(destination);
    }
    void buildIndex() throws IOException {
        List<String> groupedLines = new ArrayList<>();
        List<String> lines = new ArrayList<>();
        for (String line = input.readLine(); line != null; line = input.readLine()) {
            if(lines.size()!=5) lines.add(line);
            else {
                groupedLines.add(lines.stream().collect(Collectors.joining("\n")));
                lines.clear();
            }
        }
        ArrayList<Document> docs= groupedLines.parallelStream().map((group)->{
            Document doc=new Document();
            List<String> tokens = Tokenizer.tokenizer(group);
            for (String token : tokens) {
                doc.addWord(token);
            }
            return doc;
        }).collect(Collectors.toCollection(ArrayList::new));
        int i=0;
        for (Document doc : docs) {
            tree.addDoc(doc,i);
            i+=1;
        }
    }
    void writeIndex() throws IOException {
        FileOutputStream fos = new FileOutputStream(output);
        ObjectOutputStream oos = new ObjectOutputStream(fos);
        oos.writeObject(tree);
        oos.close();
        fos.close();
    }
    public static void main(String[] args) throws Exception {
        BuildIndex bi = new BuildIndex();
        bi.parseCliInput(args);
    }
}
