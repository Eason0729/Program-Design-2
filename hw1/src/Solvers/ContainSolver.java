package Solvers;

import java.text.CharacterIterator;
import java.text.StringCharacterIterator;

public class ContainSolver implements Solver {
    int repeat = 1;
    String content;
    String target;

    public ContainSolver(String content, String target) {
        this.content = content;
        this.target = target;
    }

    public void setRepeat(int repeat) {
        this.repeat = repeat;
    }

    public boolean solve() {
        return this.getOccur() >= this.repeat;
    }

    protected int getOccur() {
        int occur = 0;
        CharacterIterator iter = new StringCharacterIterator(this.content);

        while (iter.current() != CharacterIterator.DONE) {
            int idx = iter.getIndex();
            if (this.checkOccur(idx)) {
                occur++;
                iter.setIndex(idx + this.target.length());
            } else {
                iter.next();
            }
        }

        return occur;
    }

    private boolean checkOccur(int idx) {
        CharacterIterator target = new StringCharacterIterator(this.target);
        CharacterIterator iter = new StringCharacterIterator(this.content, idx);

        while (iter.current() != CharacterIterator.DONE) {
            if (iter.current() != target.current()) return false;
            else if (target.next() == CharacterIterator.DONE) return true;
            iter.next();
        }
        return false;
    }
}
