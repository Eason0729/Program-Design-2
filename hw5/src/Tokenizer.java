import java.text.CharacterIterator;
import java.text.StringCharacterIterator;
import java.util.ArrayList;
import java.util.List;

public class Tokenizer {
    private static boolean isDelimiter(char c) {
        return !(('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z'));
    }

    public static List<String> tokenizer(String input) {
        ArrayList<String> list = new ArrayList<>();
        StringBuilder buffer = new StringBuilder();

        StringCharacterIterator iter = new StringCharacterIterator(input);
        while (iter.current() != CharacterIterator.DONE) {
            char c = iter.current();
            if (isDelimiter(c)) {
                if (!buffer.isEmpty()) {
                    list.add(buffer.toString().toLowerCase());
                    buffer.setLength(0);
                }
            } else buffer.append(c);
            iter.next();
        }
        if (!buffer.isEmpty()) list.add(buffer.toString().toLowerCase());
        return list;
    }
}
