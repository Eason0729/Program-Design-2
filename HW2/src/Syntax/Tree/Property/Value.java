package Syntax.Tree.Property;

import Syntax.Exportable;
import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;

/**
 * Value
 * <br>
 * {@code typeName varName}
 */
public class Value implements Exportable {
    public Ident typeName;
    public Ident varName;

    public Value(PeekableTokenizer tokenizer) throws Exception {
        this.typeName = tokenizer.next();
        this.varName = tokenizer.next();
        this.typeName.errReserved("typeName");
        this.varName.errReserved("varName");
    }

    @Override
    public String export() {
        return String.format("%s %s", this.typeName.token, this.varName.token);
    }
}
