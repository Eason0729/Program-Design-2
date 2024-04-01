package Solvers;

import java.text.CharacterIterator;
import java.text.StringCharacterIterator;

public class PalindromeSolver implements Solver {
    String content;

    public PalindromeSolver(String content) {
        this.content = content;
    }

    public boolean solve() {
        CharacterIterator iter_start = new StringCharacterIterator(this.content);
        CharacterIterator iter_end = new StringCharacterIterator(this.content);
        iter_end.last();

        for (int i = 0; i < (this.content.length() / 2); i++) {
            if (iter_start.current() != iter_end.current()) return false;
            iter_start.next();
            iter_end.previous();
        }
        return true;
    }
}
