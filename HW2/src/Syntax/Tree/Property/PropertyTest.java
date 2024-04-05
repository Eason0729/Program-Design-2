package Syntax.Tree.Property;

import Syntax.Token.PeekableTokenizer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class PropertyTest {
    private static Stream<Arguments> testData() {
        return Stream.of(
                arguments("+myFn(int a,) myClass", Visibility.Public),
                arguments("-int myVal", Visibility.Private)
        );
    }

    @ParameterizedTest
    @MethodSource("testData")
    void test(String content, Visibility vis) throws Exception {
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        Property prop = new Property(tokenizer);

        assertEquals(vis, prop.vis);
    }
}