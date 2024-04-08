package Syntax.Token;

import java.io.Reader;
import java.util.ArrayList;

public class PeekableTokenizer {
    private final ArrayList<Ident> next = new ArrayList<>();
    private Tokenizer scan;

    public PeekableTokenizer(String source) {
        scan = new Tokenizer(source);
    }

    public PeekableTokenizer(Reader reader) {
        scan = new Tokenizer(reader);
    }

    private void retrieveOne() {
        Ident token = scan.next();
        if (token == null) scan = null;
        else next.add(token);
    }

    public Ident next() {
        if (next.isEmpty()) {
            if (scan == null) return null;
            this.retrieveOne();
            return this.next();
        }
        Ident current = next.get(0);
        next.remove(0);
        return current;
    }

    public boolean hasNext() {
        return this.peek() != null;
    }

    /**
     * peek next {@code Ident}
     *
     * @return indentation
     */
    public Ident peek() {
        return this.peek(0);
    }

    /**
     * peek next {@code size} {@code Ident}
     *
     * @param size amount of ident to skip
     * @return indentation
     */
    public Ident peek(int size) {
        while (next.size() < size + 1) {
            if (scan == null) return null;
            this.retrieveOne();
        }
        return next.get(size);
    }
}