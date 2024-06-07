package Search;

import Index.Documents;
import Index.Tree;

import java.util.ArrayList;
import java.util.HashMap;

enum Mode {
    Single,
    And,
    Or
}

public class Request {
    Mode mode = Mode.Single;
    HashMap<String, Integer> strategy = new HashMap<>();

    public Request(String line) {
        String[] tokens = line.split("\\s+");
        if (tokens.length > 2) mode = switch (tokens[1]) {
            case "AND" -> Mode.And;
            case "OR" -> Mode.Or;
            default -> Mode.Single;
        };

        for (int i = 0; i < tokens.length; i += 2) {
            if (strategy.containsKey(tokens[i])) {
                strategy.put(tokens[i], strategy.get(tokens[i]) + 1);
            } else {
                strategy.put(tokens[i], 1);
            }
        }
    }
    public int[] lookup(Tree tree, int max){
        HashMap<Integer, Double> tfidfs = new HashMap<>();
        HashMap<Integer, Integer> occurance=new HashMap<>();
        for (String word : strategy.keySet()) {
            int factor=strategy.get(word);
            Documents docs=tree.get(word);
            if(docs==null) continue;
            HashMap<Integer,Double> tfidfSet =docs.TFIDF(tree.word_counts,tree.totalDocs);
            for(int doc:tfidfSet.keySet()){
                if(!tfidfs.containsKey(doc)){
                    tfidfs.put(doc,0.0);
                    occurance.put(doc,0);
                }
                tfidfs.put(doc,tfidfs.get(doc)+tfidfSet.get(doc)*factor);
                occurance.put(doc,occurance.get(doc)+1);
            }
        }
        int required_occurance = switch (mode) {
            case And -> strategy.size();
            case Or -> 0;
            case Single -> 0;
        };
        ArrayList<Integer> result = new ArrayList<>();
        for(int doc:tfidfs.keySet()){
            if(occurance.get(doc)>=required_occurance){
                result.add(doc);
            }
        }
        result.sort((a,b)->Double.compare(tfidfs.get(b),tfidfs.get(a)));
        int[] res=new int[max];
        for(int i=0;i<max;i++){
            if(i<result.size()) res[i]=result.get(i);
            else res[i]=-1;
        }
        return res;
    }
}
