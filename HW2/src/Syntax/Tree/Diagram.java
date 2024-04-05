package Syntax.Tree;

import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;

import java.util.ArrayList;

/**
 * Class Diagram
 * <br>
 * {@code ClassDiagram {Statment}...}
 */
public class Diagram {
    public ArrayList<Statment> statments = new ArrayList<>();

    public Diagram(PeekableTokenizer tokenizer) throws Exception {
        Ident token = tokenizer.next();
        if (!token.token.equals("classDiagram"))
            throw token.throwError(String.format("Error: expect \"classDiagram\", found %s", token.token));

        while (tokenizer.hasNext()) statments.add(new Statment(tokenizer));
    }
}
