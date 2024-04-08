package Syntax.Token;

import java.text.CharacterIterator;
import java.text.StringCharacterIterator;

/**
 * Indentation, which contain info about span and token
 */
public class Ident extends Span {
    public String token = "";

    public Ident(int lineNo, int start, String token) {
        super(lineNo, start);
        this.token = token;
    }

    public Ident(Span span, String token) {
        super(span.lineNo, span.start);
        this.token = token;
    }

    public Ident(String token) {
        this.token = token;
    }

    public Exception throwError(String msg) {
        return new Exception(String.format("%s at %s", msg, this.getLocation()));
    }

    public boolean isReserved() {
        CharacterIterator[] progress = Tokenizer.delimiters;

        for (CharacterIterator iter : progress) {
            if ((iter.getEndIndex() + 1) != token.length()) continue;
            CharacterIterator ident = new StringCharacterIterator(this.token);
            while (iter.current() == ident.current()) {
                iter.next();
                ident.next();
            }
            if (iter.current() == CharacterIterator.DONE) return true;
        }
        return false;
    }

    public void errReserved(String msg) throws Exception {
        if (this.isReserved()) {
            throw this.throwError(String.format("Error: expect \"%s\", found \"%s\"(keyword)", msg, this.token));
        }
    }
}
