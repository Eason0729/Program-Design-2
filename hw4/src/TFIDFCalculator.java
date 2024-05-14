import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.FileReader;
import java.io.FileWriter;
import java.util.Arrays;

public class TFIDFCalculator {
    public static void main(String[] args) throws Exception {
        String documentSource = args[0];
        String testCasesSource = args[1];

        System.out.println("documentSource = " + documentSource);
        System.out.println("testCasesSource = " + testCasesSource);

        Documents documents = new Documents(documentSource);

        BufferedReader reader = new BufferedReader(new FileReader(testCasesSource));

        String[] terms = reader.readLine().split(" ");
        Integer[] docIndex = Arrays.stream(reader.readLine().split(" ")).map(Integer::parseInt).toArray(Integer[]::new);

        BufferedWriter writer = new BufferedWriter(new FileWriter("output.txt"));

        for (int i = 0; i < terms.length; i++)
            writer.write(String.format("%.5f ", documents.tfidf(terms[i], documents.docs.get(docIndex[i]))));

        writer.close();
    }
}
