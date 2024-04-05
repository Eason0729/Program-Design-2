package Syntax.Tree.Property;

import Syntax.Token.PeekableTokenizer;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class MethodTest {
    private static Stream<Arguments> parseData() {
        return Stream.of(
                arguments("myFn(int myArg1, myClass myArg2,) myRetClass,,,", "myFn", new String[]{"int", "myClass"},
                        new String[]{"myArg1", "myArg2"}, "myRetClass"),
                arguments("myFn'(int,) string,,,", "myFn'", new String[]{"int"},
                        new String[]{","}, "string"),
                arguments("myFn(int myArg1)", "myFn", new String[]{"int"},
                        new String[]{"myArg1"}, "void"),
                arguments("myFn(int myArg1)\nclass myPlaceHolderClass", "myFn", new String[]{"int"},
                        new String[]{"myArg1"}, "void"),
                arguments("myFn(int myArg1)\n+myFn2(int myArg1) Ret3", "myFn", new String[]{"int"},
                        new String[]{"myArg1"}, "void")
        );
    }

    private static Stream<Arguments> exportData() {
        return Stream.of(
                arguments("myFn(int myArg1,) myRetClass", "myRetClass myFn(int myArg1) {;}"),
                arguments("setVal(int myArg1) myRetClass", "myRetClass setVal(int myArg1) {\n    this.val = myArg1;\n}"),
                arguments("getABC(int myArg1,) myRetClass", "myRetClass getABC(int myArg1) {\n    return aBC;\n}")
        );
    }

    @ParameterizedTest
    @MethodSource("parseData")
    void parse(String content, String fnName, String[] argTypesExp, String[] argNamesExp, String retType) throws Exception {
        // because ArgumentTest doesn't test list of args, we check it here
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        Method method = new Method(tokenizer);

        assertEquals(fnName, method.fnName.token);
        assertEquals(retType, method.retType.token);

        ArrayList<String> argTypes = new ArrayList<>();
        ArrayList<String> argNames = new ArrayList<>();
        for (Argument argc : method.arguments) {
            argTypes.add(argc.argType.token);
            argNames.add(argc.argName.token);
        }

        assertEquals(Arrays.asList(argTypesExp), argTypes);
        assertEquals(Arrays.asList(argNamesExp), argNames);
    }

    @ParameterizedTest
    @MethodSource("exportData")
    void export(String content, String result) throws Exception {
        PeekableTokenizer tokenizer = new PeekableTokenizer(content);
        Method method = new Method(tokenizer);

        assertEquals(result, method.export());
    }
}