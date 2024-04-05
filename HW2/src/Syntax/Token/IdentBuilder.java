package Syntax.Token;

import java.util.regex.Pattern;

public class IdentBuilder extends Span {
    public StringBuilder token = new StringBuilder();

    public IdentBuilder() {
    }

    public IdentBuilder(Span span) {
        super(span.lineNo, span.start);
    }

    public void append(char c) {
        token.append(c);
    }

    public Ident substring(int start, int end) {
        return new Ident(this.lineNo, this.start + start, this.token.substring(start, end).replaceAll(Pattern.quote("#%$^"), "<|--"));
    }

    public Ident build() {
        return new Ident(this, token.toString().replaceAll(Pattern.quote("#%$^"), "<|--"));
    }

    public Exception throwError(String msg) {
        return new Exception(String.format("%s at %s", msg, this.getLocation()));
    }

    public boolean isEmpty() {
        return this.token.isEmpty();
    }

    public int length() {
        return this.token.length();
    }
}
