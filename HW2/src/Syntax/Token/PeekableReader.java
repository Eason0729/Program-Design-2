package Syntax.Token;

import java.io.IOException;
import java.io.Reader;
import java.io.StringReader;
import java.text.CharacterIterator;

public class PeekableReader {
    private final Reader reader;
    private char value = CharacterIterator.DONE;

    public PeekableReader(Reader reader) {
        this.reader = reader;
        this.retrieveOne();
    }

    public PeekableReader(String content) {
        this.reader = new StringReader(content);
        this.retrieveOne();
    }

    private void retrieveOne() {
        try {
            this.value = (char) this.reader.read();
        } catch (IOException e) {
        }
    }

    public boolean hasNext() {
        return value != CharacterIterator.DONE;
    }

    public char current() {
        return value;
    }

    public char next() {
        this.retrieveOne();
        return value;
    }
}
