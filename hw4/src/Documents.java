import trie.Trie;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.io.Reader;
import java.util.ArrayList;

public class Documents {
    private final Trie occurrences = new Trie();
    public ArrayList<Document> docs = new ArrayList<>();

    public Documents(String filename) throws IOException {
        this(new BufferedReader(new FileReader(filename)));
    }

    public Documents(Reader input) throws IOException {
        BufferedReader reader = new BufferedReader(input);
        while (true) {
            Document doc = new Document(reader);
            if (doc.isEmpty()) break;
            occurrences.merge(doc.tree);
            docs.add(doc);
        }
    }

    public double inverseDocumentFrequency(String term) {
        return Math.log(docs.size() / (double) occurrences.get(term));
    }

    public double tfidf(String term, Document doc) {
        return doc.termFrequency(term) * inverseDocumentFrequency(term);
    }
}
