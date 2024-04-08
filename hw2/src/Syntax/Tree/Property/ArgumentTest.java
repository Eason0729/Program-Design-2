package Syntax.Tree.Property;

import Syntax.Token.PeekableTokenizer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class ArgumentTest {
    private static Stream<Arguments> testData() {
        return Stream.of(
                arguments("myClass efg", "myClass", "efg"),
                arguments("int abc,)", "int", "abc")
        );
    }

    @ParameterizedTest
    @MethodSource("testData")
    void test(String content, String argType, String argName) throws Exception {
        System.out.println("Source:");
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        Argument arg = new Argument(tokenizer);

        assertEquals(arg.argName.token, argName);
        assertEquals(arg.argType.token, argType);
    }
}