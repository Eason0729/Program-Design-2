package Syntax.Token;

public class Span {
    protected int lineNo = 0;
    protected int start = 0;

    public Span() {
    }

    public Span(int lineNo, int start) {
        this.lineNo = lineNo;
        this.start = start;
    }

    public void nextLine() {
        this.lineNo++;
        this.start = 0;
    }

    public void nextCharacter() {
        this.start++;
    }

    public void setLocationDelta(int delta) {
        this.start += delta;
    }

    public String getLocation() {
        return String.format("%d:%d", lineNo, start);
    }

    public Span clone() {
        return new Span(lineNo, start);
    }
}
