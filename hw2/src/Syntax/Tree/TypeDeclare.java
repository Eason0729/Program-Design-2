package Syntax.Tree;

import Syntax.Exportable;
import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;
import Syntax.Tree.Property.Property;

import java.util.ArrayList;

/**
 * TypeDeclaration
 * <br>
 */
public class TypeDeclare implements Exportable {
    public Ident typeName;
    public ArrayList<Property> properties = new ArrayList<>();

    /**
     * construct new TypeDeclare with fake token
     * @param typeName name
     */
    public TypeDeclare(String typeName) {
        this.typeName=new Ident(typeName);
    }
    public TypeDeclare(PeekableTokenizer tokenizer) throws Exception {
        Ident token = tokenizer.next();

        if (!token.token.equals("class"))
            throw token.throwError(String.format("Error: expect keyword \"class\", found \"%s\"", token.token));

        this.typeName = tokenizer.next();
        this.typeName.errReserved("typeName");

        if (tokenizer.hasNext() && tokenizer.peek().token.equals("{")) {
            tokenizer.next();
            while (!tokenizer.peek().token.equals("}")) this.properties.add(new Property((tokenizer)));
            tokenizer.next();
        }
    }

    public void addProperty(Property prop) {
        this.properties.add(prop);
    }

    public void merge(ArrayList<Property> props) {
        this.properties.addAll(props);
    }

    @Override
    public String export() throws Exception {
        StringBuilder propExport = new StringBuilder();

        for (Property prop : properties) propExport.append("\n    ").append(prop.export().replaceAll("\n", "\n    "));

        return String.format("public class %s {%s\n}", typeName.token, propExport);
    }
}
