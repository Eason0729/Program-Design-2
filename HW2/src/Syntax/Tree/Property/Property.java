package Syntax.Tree.Property;

import Syntax.Exportable;
import Syntax.Token.Ident;
import Syntax.Token.PeekableTokenizer;

/**
 * Property
 * <br>
 * {@code vis {Value|Method}}
 */
public class Property implements Exportable {
    public Visibility vis;
    public Value value = null;
    public Method method = null;

    public Property(PeekableTokenizer tokenizer) throws Exception {
        Ident vis = tokenizer.next();
        switch (vis.token.charAt(0)) {
            case '+' -> this.vis = Visibility.Public;
            case '-' -> this.vis = Visibility.Private;
            default -> throw vis.throwError("Error: expect visibility(+,-)");
        }

        if (tokenizer.peek(1).token.equals("(")) this.method = new Method(tokenizer);
        else this.value = new Value(tokenizer);
    }

    public boolean isMethod() {
        return method != null;
    }

    public boolean isValue() {
        return value != null;
    }

    @Override
    public String export() throws Exception {
        String visExport = switch (vis) {
            case Public -> "public";
            case Private -> "private";
        };

        if (this.isMethod()) return String.format("%s %s", visExport, this.method.export());
        if (this.isValue()) return String.format("%s %s;", visExport, this.value.export());

        throw new Exception("unsupported");
    }
}
