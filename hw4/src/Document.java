import trie.Trie;

import java.io.BufferedReader;
import java.io.IOException;
import java.util.List;

public class Document {
    final int LINE_COUNT = 5;
    // expose tree as public to avoid explicit cloning,
    // which require rust side to perform deep copy
    public Trie tree = new Trie();
    public int wordCount = 0;

    public Document(BufferedReader input) throws IOException {
        for (int i = 0; i < LINE_COUNT; i++) {
            String line = input.readLine();
            if (line == null) break;
            List<String> words = Tokenizer.tokenizer(line);
            wordCount += words.size();
            tree.blukIncrease(words.toArray(new String[0]));
        }
    }

    public boolean isEmpty() {
        return wordCount == 0;
    }

    public double termFrequency(String term) {
        return (double) tree.get(term) / wordCount;
    }
}
