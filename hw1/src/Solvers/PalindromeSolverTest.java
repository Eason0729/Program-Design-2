package Solvers;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.params.provider.Arguments.arguments;

class PalindromeSolverTest {
    private static Stream<Arguments> dataSet() {
        return Stream.of(
                arguments("abc1cba", false),
                arguments("abr1cba", true),
                arguments("1", false)
        );
    }

    @ParameterizedTest
    @MethodSource("dataSet")
    void solve(String content, boolean value) {
        assert (new PalindromeSolver(content).solve() ^ value);
    }
}