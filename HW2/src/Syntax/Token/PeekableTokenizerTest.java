package Syntax.Token;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class PeekableTokenizerTest {
    @Test
    public void basic() {
        PeekableTokenizer tokenizer = new PeekableTokenizer("myFn(int myArg1, myClass myArg2,) myRetClass,,");
        String[] tokens = {
                "myFn", "(", "int", "myArg1", ",", "myClass", "myArg2", ",", ")", "myRetClass", ",", ",",
        };
        for (int i = 0; i < tokens.length; i++) {
            System.out.printf("Peeking %d%n", i);
            for (int j = i; j < tokens.length; j++) assertEquals(tokenizer.peek(j - i).token, tokens[j]);
            assertEquals(tokenizer.next().token, tokens[i]);
        }
        for (int i = 0; i < 10; i++) assertNull(tokenizer.next());
    }

    @Test
    public void peekable() {
        PeekableTokenizer tokenizer = new PeekableTokenizer("myFn(int myArg1) myRetClass\n");
        for (int i = 0; i < 5; i++) tokenizer.next();
        assertNull(tokenizer.peek(1));
        assertNotNull(tokenizer.next());
    }
}