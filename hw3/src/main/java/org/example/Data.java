package org.example;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.Reader;
import java.io.StringReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;

public class Data {
    public ArrayList<String> name = new ArrayList<>();
    public ArrayList<ArrayList<Double>> value = new ArrayList<>();
    public int len = 30;

    public Data(String content, int len) throws IOException {
        this.len = len;
        this.loadData(new BufferedReader(new StringReader(content)));
    }

    public Data(Reader reader) throws IOException {
        this.loadData(new BufferedReader(reader));
    }

    public void loadData(BufferedReader reader) throws IOException {
        Collections.addAll(name, reader.readLine().split(","));
        for (int i = 1; i <= len; i++)
            this.value.add(new ArrayList<>(Arrays.asList(Arrays.stream(reader.readLine().split(",")).map(Double::valueOf).toArray(Double[]::new))));
    }
}
