package Syntax.Tree;

import Syntax.Token.PeekableTokenizer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class DiagramTest {
    private static Stream<Arguments> testData() {
        return Stream.of(
                arguments("""
                        classDiagram
                        class BankAccount
                        BankAccount : -String owner
                        BankAccount : +setOwner(String owner) void
                        """, 3),
                arguments("""
                        classDiagram
                            class Person
                            Person : +introduceSelf(String name) void

                            class Student {
                                +String studentID
                                +study() void
                            }
                            class Teacher {
                                +String teacherID
                                +teach() void
                            }
                            Person : -int age
                            Person : -String name

                            class Student {
                                -int number
                                -Teacher correspondingTeacher
                            }
                            """, 7),
                arguments("""
                        classDiagram
                        class MyClass
                        """, 1)
        );
    }

    @ParameterizedTest
    @MethodSource("testData")
    void test(String content, int n) throws Exception {
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        Diagram diagram = new Diagram(tokenizer);

        assertEquals(n, diagram.statments.size());
    }
}