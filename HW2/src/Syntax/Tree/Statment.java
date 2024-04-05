package Syntax.Tree;

import Syntax.Token.PeekableTokenizer;

/**
 * Statment
 * <br>
 * {@code {TypeDeclare|propertyEntry}}
 */
public class Statment {
    public TypeDeclare typeDeclare = null;
    public PropertyEntry propertyEntry = null;

    public Statment(PeekableTokenizer tokenizer) throws Exception {
        if (tokenizer.peek().token.equals("class")) this.typeDeclare = new TypeDeclare((tokenizer));
        else if (tokenizer.peek(1).token.equals(":")) this.propertyEntry = new PropertyEntry(tokenizer);
    }

    public boolean isTypeDeclare() {
        return this.typeDeclare != null;
    }

    public boolean isPropertyEntry() {
        return this.propertyEntry != null;
    }
}
