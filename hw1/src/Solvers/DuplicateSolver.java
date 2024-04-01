package Solvers;

import java.text.CharacterIterator;
import java.text.StringCharacterIterator;

public class DuplicateSolver implements Solver {
    String content;

    public DuplicateSolver(String content) {
        this.content = content;
    }

    public boolean solve() {
        CharacterIterator[] target = {
                new StringCharacterIterator("a"),
                new StringCharacterIterator("bb")
        };
        CharacterIterator iter = new StringCharacterIterator(this.content);

        int i = 0;

        while (iter.current() != CharacterIterator.DONE) {
            if (iter.current() != target[i].current())
                target[i].first();
            else if (target[i].next() == CharacterIterator.DONE)
                i++;
            if (i == target.length) return true;
            iter.next();
        }
        return false;
    }
}
