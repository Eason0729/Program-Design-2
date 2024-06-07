import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.HexFormat;

class HexInputStream extends InputStream {
    private final InputStream in;

    public HexInputStream(String path) throws IOException {
        final byte[] magicHint = ("// " + "Embed" + "DyLib: ").getBytes(StandardCharsets.UTF_8);

        FileInputStream file = new FileInputStream(path);

        // read until the magic hint
        int i = 0;
        while (i < magicHint.length) {
            int b = file.read();
            if (b == -1) {
                throw new EOFException("Cannot find magic hint");
            }
            if (b == magicHint[i]) {
                i++;
            } else {
                i = 0;
            }
        }
        in = new BufferedInputStream(file);
    }

    public HexInputStream(InputStream in) {
        this.in = new BufferedInputStream(in);
    }

    public void saveToFile(String path) throws IOException {
        FileOutputStream stream = new FileOutputStream(path);
        byte[] buffer = new byte[32 * 1024];
        int bytesRead;
        while ((bytesRead = read(buffer)) != -1) {
            stream.write(buffer, 0, bytesRead);
        }
        stream.flush();
        stream.getFD().sync();
        ;
    }

    @Override
    public int read() throws IOException {
        try {
            byte[] raw = in.readNBytes(2);
            if (raw.length < 2) {
                return -1;
            }
            String str = new String(raw, StandardCharsets.UTF_8);
            return HexFormat.fromHexDigits(str, 0, 2);
        } catch (EOFException e) {
            return -1;
        }
    }
}

public class Indexer implements Serializable {
    private static final String dyLibPath = "libIndexer.so";
    private static final String dyLibEmbedSource = "Indexer.java";

    static {
        try {
            Path dyLibPathObj = Paths.get(dyLibPath).toAbsolutePath();

	    if (!new File(dyLibPath).exists()) {
                HexInputStream reader = new HexInputStream(dyLibEmbedSource);
                reader.saveToFile(dyLibPath);
                Files.setPosixFilePermissions(dyLibPathObj, PosixFilePermissions.fromString("rwxrwxr-x"));
	    }

            System.load(dyLibPathObj.toString());
        } catch (Exception ex) {
            throw new RuntimeException("Fail to load embedded dylib source: " + dyLibEmbedSource, ex);
        }
    }

    protected static native void buildIndex(String[] args);

    protected static native void TFIDFSearch(String[] args);
}
// EmbedDyLib: 766572696679206d6521
