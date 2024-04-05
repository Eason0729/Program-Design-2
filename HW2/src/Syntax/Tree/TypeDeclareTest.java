package Syntax.Tree;

import Syntax.Token.PeekableTokenizer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class TypeDeclareTest {
    private static Stream<Arguments> parseData() {
        return Stream.of(
                arguments("""
                        class Student {
                                +String studentID
                                +study() void
                            }
                        """, "Student"),
                arguments("class myClass{\n}", "myClass"),
                arguments("class myClass", "myClass")
        );
    }

    private static Stream<Arguments> exportData() {
        return Stream.of(
                arguments("""
                        class Student {
                                +String studentID
                                +study() void
                            }
                        """, "public class Student {\n    public String studentID;\n    public void study() {;}\n}"),
                arguments("""
                        class Student {
                                +String studentID
                            }
                        """, "public class Student {\n    public String studentID;\n}")
        );
    }

    @ParameterizedTest
    @MethodSource("parseData")
    void parse(String content, String typeName) throws Exception {
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        TypeDeclare typeDeclare = new TypeDeclare(tokenizer);

        assertEquals(typeDeclare.typeName.token, typeName);
        assert (!tokenizer.hasNext());
    }

    @ParameterizedTest
    @MethodSource("exportData")
    void export(String content, String expect) throws Exception {
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        TypeDeclare typeDeclare = new TypeDeclare(tokenizer);

        assertEquals(expect, typeDeclare.export());
    }
}