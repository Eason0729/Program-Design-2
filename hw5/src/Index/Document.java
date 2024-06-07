package Index;

import java.io.Serializable;
import java.util.HashMap;
import java.util.List;

public class Document implements Serializable {
    HashMap<String, Integer> word_counts = new HashMap<>();
    int word_count=0;
    int occurance = 0;
    public void addWord(String word){
        word_count +=1;
        if (word_counts.containsKey(word)) word_counts.put(word, word_counts.get(word)+1);
        else{
            word_counts.put(word, 1);
            occurance +=1;
        }
    }
    public double from_tokens(List<String> tokens){
        for (String token : tokens) addWord(token);
        return (double) word_count;
    }
}
