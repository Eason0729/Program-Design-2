package Syntax.Tree;

import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;
import Syntax.Tree.Property.Property;

/**
 * Property Entry
 * <br>
 * {@code typeName : {Property}}
 */
public class PropertyEntry {
    public Ident typeName;
    public Property prop;

    public PropertyEntry(PeekableTokenizer tokenizer) throws Exception {
        this.typeName = tokenizer.next();
        this.typeName.errReserved("typeName");

        Ident token = tokenizer.next();
        if (!token.token.equals(":"))
            throw token.throwError(String.format("Error: except keyword \":\", found %s", token.token));

        this.prop = new Property((tokenizer));
    }
}
