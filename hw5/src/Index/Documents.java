package Index;

import java.io.Serializable;
import java.util.HashMap;
import java.util.List;

public class Documents implements Serializable {
    int occurance =0;
    HashMap<Integer, Integer> word_counts=new HashMap<>();
    public void insert(int id, int word_count){
        word_counts.put(id, word_count);
        occurance +=1;
    }
    public HashMap<Integer, Double> termFrequency(List<Integer> docWordCount){
        HashMap<Integer, Double> term_frequency = new HashMap<>();
        for (int doc : word_counts.keySet())
            term_frequency.put(doc, (double) word_counts.get(doc)
                    /  (double) docWordCount.get(doc));

        return term_frequency;
    }
    public double inverseDocumentFrequency(int total_docs){
        return Math.log((double)total_docs / (double)word_counts.size());
    }
    public HashMap<Integer, Double> TFIDF(List<Integer> docWordCount,int total_docs){
        HashMap<Integer, Double> tfidf = new HashMap<>();
        HashMap<Integer, Double> term_frequency = termFrequency(docWordCount);
        double inverse_document_frequency = inverseDocumentFrequency(total_docs);
        for (int doc : term_frequency.keySet()) {
            tfidf.put(doc, term_frequency.get(doc) * inverse_document_frequency);
        }
        return tfidf;
    }
}
