package Solvers;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

class DuplicateSolverTest {
    private static Stream<Arguments> dataSet() {
        return Stream.of(
                Arguments.arguments("abb", true),
                Arguments.arguments("axgbxb", false),
                Arguments.arguments("axxbb", true),
                Arguments.arguments("YPsCPjmsp JvUKeCxbBF bVepEKBJ sCaDYHgc afs YEDcXQjECw YUDUys PJbGym", false),
                Arguments.arguments("This is a bug, but is fixed.", false)

        );
    }

    @ParameterizedTest
    @MethodSource("dataSet")
    void solve(String content, boolean value) {
        assert (!new DuplicateSolver(content).solve() ^ value);
    }

}