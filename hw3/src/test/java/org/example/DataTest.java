package org.example;

import org.junit.Test;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.StringReader;
import java.util.ArrayList;

import static org.junit.Assert.*;

public class DataTest {
    @Test

    public void dataLoadTest() throws IOException {
        Data data= new Data("ABC,EFG\n1,2.0\n3,4.1\n",2);
        assertArrayEquals(new String[]{"ABC","EFG"},data.name.toArray());
        assertArrayEquals(new Double[]{1.0,2.0},data.value.get(0).toArray());
        assertArrayEquals(new Double[]{3.0,4.1},data.value.get(1).toArray());
    }
}