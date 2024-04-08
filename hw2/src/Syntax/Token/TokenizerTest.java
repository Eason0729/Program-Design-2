package Syntax.Token;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.params.provider.Arguments.arguments;


class TokenizerTest {

    private static Stream<Arguments> nextData() {
        return Stream.of(
                arguments("class myClass{\n}", new String[]{"class", "myClass", "{", "}"}),
                arguments("abc  edf a", new String[]{"abc", "edf", "a"}),
                arguments("abca", new String[]{"abca"}),
                arguments("abc  +edf a", new String[]{"abc", "+", "edf", "a"}),
                arguments("abc  <|+--edf a", new String[]{"abc", "<", "|", "+", "-", "-", "edf", "a"}),
                arguments("abc<|--edf a", new String[]{"abc", "<|--", "edf", "a"}),
                arguments("abc-edf\na", new String[]{"abc", "-", "edf", "a"}),
                arguments("- -a", new String[]{"-", "-", "a"}),
                arguments("+myFn(int a,) myClass", new String[]{"+", "myFn", "(", "int", "a", ",", ")", "myClass"}),
                arguments("myFn(int myArg1, myClass myArg2,) myRetClass,,", new String[]{
                        "myFn", "(", "int", "myArg1", ",", "myClass", "myArg2", ",", ")", "myRetClass", ",", ",",
                })
        );
    }

    @ParameterizedTest
    @MethodSource("nextData")
    void next(String content, String[] expect) throws Exception {
        System.out.println("Source:");
        Tokenizer tokenizer = new Tokenizer(content);
        ArrayList<String> result = new ArrayList<String>();

        Ident token = tokenizer.next();
        while (token != null) {
            result.add(token.token);
            token = tokenizer.next();
        }

        assertEquals(Arrays.asList(expect), result);

        for (int i = 0; i < 10; i++) assertNull(tokenizer.next());
    }
}