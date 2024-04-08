package Syntax.Tree.Property;

import Syntax.Exportable;
import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;

/**
 * Argument of function
 * <br>
 * {@code argName argType}
 */
public class Argument implements Exportable {
    Ident argName;
    Ident argType;

    public Argument(PeekableTokenizer tokenizer) throws Exception {
        this.argType = tokenizer.next();
        this.argName = tokenizer.next();

        this.argName.errReserved("argName");
        this.argType.errReserved("argType");
    }

    @Override
    public String export() {
        return String.format("%s %s", this.argType.token, this.argName.token);
    }
}
