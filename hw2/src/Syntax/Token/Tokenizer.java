package Syntax.Token;

import java.io.Reader;
import java.text.CharacterIterator;
import java.text.StringCharacterIterator;
import java.util.LinkedList;
import java.util.Queue;
import java.util.regex.Matcher;

public class Tokenizer {
    public static final CharacterIterator[] delimiters = {
            new StringCharacterIterator("{"),
            new StringCharacterIterator("}"),
            new StringCharacterIterator("#%$^"),
            new StringCharacterIterator("+"),
            new StringCharacterIterator("-"),
            new StringCharacterIterator("|"),
            new StringCharacterIterator(":"),
            new StringCharacterIterator("("),
            new StringCharacterIterator(")"),
            new StringCharacterIterator(","),
    };
    private static final char[] splitter = {'\n', ' '};
    Queue<Ident> tokens = new LinkedList<>();
    CharacterIterator[] progress = delimiters;
    PeekableReader content;
    IdentBuilder remain = new IdentBuilder();

    public Tokenizer(String content) {
        this.content = new PeekableReader(
                content.replaceAll("<\\|--", Matcher.quoteReplacement("#%$^"))
        );
    }

    public Tokenizer(Reader reader) {
        this.content = new PeekableReader(reader);
    }

    /**
     * Reset internal progress
     */
    private void resetProgress() {
        this.progress = Tokenizer.delimiters;
        this.remain = new IdentBuilder(this.remain);
    }

    /**
     * Check splitter(character that should be removed, and also work as delimiter),
     * add {@code current} to {@code remain} if it's not a splitter
     *
     * @param current character to check
     */
    private void checkSplitter(char current) {
        if (current == '\n') remain.nextLine();
        else remain.nextCharacter();
        for (char c : Tokenizer.splitter) {
            if (c == current) {
                if (!this.remain.isEmpty()) tokens.add(this.remain.build());
                this.resetProgress();
                return;
            }
        }
        this.remain.append(current);
    }

    /**
     * poll tokens, and replace <code><|--</code> workaround, it returns null if empty
     *
     * @return token
     */
    private Ident checkToken() {
        if (this.content.current() == CharacterIterator.DONE && this.remain != null) {
            if (!this.remain.isEmpty()) tokens.add(this.remain.build());
            remain = null;
        }
        if (tokens.isEmpty()) return null;
        return tokens.remove();
    }

    /**
     * Check for Delimiter(syntax-meaningful character, and also work as delimiter)
     *
     * @param current character to check
     */
    private void checkDelimiter(char current) {
        for (CharacterIterator iter : this.progress) {
            if (iter.current() != current) iter.first();
            else if (iter.next() == CharacterIterator.DONE) {
                int split = remain.length() - iter.getEndIndex();

                if (split != 0) this.tokens.add(remain.substring(0, split));
                this.tokens.add(remain.substring(split, remain.length()));

                this.resetProgress();
                return;
            }
        }
    }

    /**
     * Attempt to check next token, return null if EOF
     *
     * @return next token
     */
    public Ident next() {
        while (this.content.current() != CharacterIterator.DONE && this.tokens.isEmpty()) {
            char current = this.content.current();
            this.content.next();
            this.checkSplitter(current);
            if (!this.tokens.isEmpty()) break;
            this.checkDelimiter(current);
        }
        return this.checkToken();
    }
}
