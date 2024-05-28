import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class IndexerTest {

    @Test
    void test() {
        Indexer.buildIndex(new String[]{"-i", "src/test/resources/indexer/input.txt", "-d", "src/test/resources/indexer/output"});
    }
}