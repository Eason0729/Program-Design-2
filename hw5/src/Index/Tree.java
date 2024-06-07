package Index;

import java.io.Serializable;
import java.util.ArrayList;
import java.util.HashMap;

public class Tree implements Serializable {
    HashMap<String, Documents> tree = new HashMap<>();
    public int totalDocs = 0;
    public ArrayList<Integer> word_counts = new ArrayList<>();
    public void addDoc(Document doc, int id){
        totalDocs +=1;

        if (word_counts.size() > id) word_counts.set(id, doc.word_count);
        word_counts.add(doc.word_count);

        for (String word : doc.word_counts.keySet()) {
            if (!tree.containsKey(word)) tree.put(word, new Documents());
            tree.get(word).insert(id, doc.word_counts.get(word));
        }
    }
    public Documents get(String word){
        return tree.get(word);
    }
    public HashMap<Integer, Double> tfidf(String word){
        HashMap<Integer, Double> tfidf = new HashMap<>();
        if (tree.containsKey(word)){
            Documents docs = tree.get(word);
            HashMap<Integer, Double> term_frequency = docs.termFrequency(word_counts);
            double inverse_document_frequency = docs.inverseDocumentFrequency(totalDocs);
            for (int doc : term_frequency.keySet()) {
                tfidf.put(doc, term_frequency.get(doc) * inverse_document_frequency);
            }
        }
        return tfidf;
    }

}
