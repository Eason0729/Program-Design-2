package Solvers;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class ContainSolverTest {
    private static Stream<Arguments> dataSet() {
        return Stream.of(
                arguments("aaa", "a", 3),
                arguments("aaab", "aab", 1),
                arguments("aaa", "aaaaa", 0),
                arguments("aaggbhddggbbbbbgabbbb", "gb", 2),
                arguments("aaaaa", "aa", 2),
                arguments("odfdsb abababagh ", "aba", 2)
        );
    }

    @ParameterizedTest
    @MethodSource("dataSet")
    void occur(String content, String target, int value) {
        ContainSolver solver = new ContainSolver(content, target);
        assertEquals(value, solver.getOccur());
    }

    @ParameterizedTest
    @MethodSource("dataSet")
    void solve(String content, String target, int value) {
        ContainSolver solver = new ContainSolver(content, target);
        solver.setRepeat(value);
        assert (solver.solve());
    }
}