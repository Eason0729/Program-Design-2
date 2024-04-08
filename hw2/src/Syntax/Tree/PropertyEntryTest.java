package Syntax.Tree;

import Syntax.Token.PeekableTokenizer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class PropertyEntryTest {
    private static Stream<Arguments> testData() {
        return Stream.of(
                arguments("Person : \n -int age\n", "Person"),
                arguments("Person : +study () void\n", "Person")
        );
    }

    @ParameterizedTest
    @MethodSource("testData")
    void test(String content, String typeName) throws Exception {
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        PropertyEntry typeDeclare = new PropertyEntry(tokenizer);

        assertEquals(typeDeclare.typeName.token, typeName);
        assert (!tokenizer.hasNext());
    }
}