import Syntax.Token.PeekableTokenizer;
import Syntax.Tree.Diagram;
import Syntax.Tree.PropertyEntry;
import Syntax.Tree.Statment;
import Syntax.Tree.TypeDeclare;

import java.io.*;
import java.util.HashMap;

public class CodeGenerator {
    private final HashMap<String, TypeDeclare> classes = new HashMap<>();
    PeekableTokenizer tokenizer;

    public CodeGenerator(String[] args) {
        this.tokenizer = new PeekableTokenizer(getReader(args));
    }
    private void insert(PropertyEntry entry){
        String typeName=entry.typeName.token;
        if(!classes.containsKey(typeName)) classes.put(typeName,new TypeDeclare(typeName));
        classes.get(typeName).addProperty(entry.prop);
    }
    private void insert(TypeDeclare declare){
        String typeName=declare.typeName.token;
        if(classes.containsKey(typeName))classes.get(typeName).merge(declare.properties);
        else classes.put(typeName,declare);
    }
    private static Reader getReader(String[] args) {
        if (args.length == 0) {
            System.err.println("請輸入檔案名稱");
            System.exit(0);
            return null;
        }
        String fileName = args[0];
        System.out.println("File name: " + fileName);

        try {
            return new BufferedReader(new FileReader(fileName));
        } catch (IOException e) {
            System.err.println("無法讀取文件 " + fileName);
            System.exit(0);
            return null;
        }
    }

    public void parse() throws Exception {
        Diagram diagram = new Diagram(tokenizer);
        for (Statment stat : diagram.statments) {
            if (stat.isTypeDeclare()) this.insert(stat.typeDeclare);
            if (stat.isPropertyEntry())this.insert(stat.propertyEntry);
        }
    }

    public void generate() throws Exception {
        for (TypeDeclare typeDeclare : classes.values()) {
            String typeName = typeDeclare.typeName.token;
            File file = new File(typeName + ".java");
            if (!file.exists()) file.createNewFile();
            FileWriter writer = new FileWriter(file);
            writer.write(typeDeclare.export());
            writer.close();
        }
    }

    public static void main(String[] args) throws Exception {
        CodeGenerator generator = new CodeGenerator(args);

        generator.parse();
        generator.generate();
    }
}
