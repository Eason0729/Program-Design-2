package org.example;

import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;

import java.io.*;
import java.nio.file.Files;
import java.text.DecimalFormat;
import java.util.ArrayList;

public class HtmlParser {
    public static void main(String[] args) throws Exception {
        if (args.length < 1) System.err.println("Usage: java HtmlParser <mode>");
        if (args[0].equals("0")) HtmlParser.crawl();
        switch (args[1]) {
            case "0" -> HtmlParser.task0();
            case "1" -> HtmlParser.task1(args);
            case "2" -> HtmlParser.task2(args);
            case "3" -> HtmlParser.task3(args);
            case "4" -> HtmlParser.task4(args);
        }
    }

    public static String ff(double s) {
        DecimalFormat formatter = new DecimalFormat("#.##");
        return formatter.format(s);
    }

    public static ArrayList<Double> getStock(String stock) throws IOException {
        Data data = new Data(new FileReader(new File("data.csv")));
        int index = data.name.indexOf(stock);
        ArrayList<Double> price = new ArrayList<>();
        for (int i = 0; i < data.len; i++) price.add(data.value.get(i).get(index));
        return price;
    }

    public static void output(String content) throws IOException {
        BufferedWriter writer = new BufferedWriter(new FileWriter(new File("output.csv"), true));
        writer.write(content);
        writer.flush();
        writer.close();
    }

    public static void task0() throws IOException {
        // copy data.csv to output.csv
        Files.copy(new File("data.csv").toPath(), new File("output.csv").toPath());
    }

    public static void task1(String[] args) throws IOException {
        String stock = args[2];
        int start = Integer.parseInt(args[3]);
        int end = Integer.parseInt(args[4]);

        ArrayList<Double> price = HtmlParser.getStock(stock);
        if((end-start)<3){
            HtmlParser.output(stock + "," + start + "," + end + "\n\n");
            return;
        }
        ArrayList<Double> prefixSum = new ArrayList<>();
        prefixSum.add(0.0);
        for (int i = 0; i < price.size(); i++) {
            prefixSum.add(prefixSum.get(i) + price.get(i));
        }

        ArrayList<String> result = new ArrayList<>();
        for(int i = start-1; i < end-4; i++){
            result.add(ff((prefixSum.get(i+5) - prefixSum.get(i))/5));
        }

        HtmlParser.output(stock + "," + start + "," + end + "\n"+String.join(",",result)+"\n");
    }

    public static void task2(String[] args) throws IOException {
        String stock = args[2];
        int start = Integer.parseInt(args[3]);
        int end = Integer.parseInt(args[4]);

        ArrayList<Double> price = HtmlParser.getStock(stock);
        // compute standard deviation for stock price in range start to end
        double sum = 0;
        for (int i = start - 1; i < end; i++) sum += price.get(i);
        double mean = sum / (end - start + 1);
        sum = 0;
        for (int i = start - 1; i < end; i++) sum += Math.pow(price.get(i) - mean, 2);
        double std = Math.sqrt(sum / (end - start));

        HtmlParser.output(stock + "," + start + "," + end + "\n" + ff(std) + "\n");
    }

    public static void task3(String[] args) throws IOException {
        int start = Integer.parseInt(args[3]);
        int end = Integer.parseInt(args[4]);

        // compute top 3 stock with highest standard deviation
        // 1. load data
        Data data = new Data(new FileReader(new File("data.csv")));
        // 2. compute standard deviation for each stock
        ArrayList<Double> std = new ArrayList<>();
        for (int i = 0; i < data.name.size(); i++) {
            double sum = 0;
            for (int j = start - 1; j < end; j++) sum += data.value.get(j).get(i);
            double mean = sum / (end - start + 1);
            sum = 0;
            for (int j = start - 1; j < end; j++) sum += Math.pow(data.value.get(j).get(i) - mean, 2);
            std.add(Math.sqrt(sum / (end - start)));
        }
        // 3. sort by standard deviation
        ArrayList<Integer> index = new ArrayList<>();
        for (int i = 0; i < data.name.size(); i++) index.add(i);
        index.sort((a, b) -> std.get(b).compareTo(std.get(a)));
        // 4. output top 1 stock name
        HtmlParser.output(data.name.get(index.get(0)) + "," + data.name.get(index.get(1)) + ","
                + data.name.get(index.get(2)) + "," + start + "," + end + "\n"
                + ff(std.get(index.get(0))) + "," + ff(std.get(index.get(1))) + ","
                + ff(std.get(index.get(2))) + "\n");
    }

    public static void task4(String[] args) throws IOException {
        String stock = args[2];
        int start = Integer.parseInt(args[3]);
        int end = Integer.parseInt(args[4]);

        // compute linear regression for stock price in range start to end
        ArrayList<Double> price = HtmlParser.getStock(stock);
        double averagePrice = 0;
        double averageDay = (start + end) / 2.0;
        for (int i = start - 1; i < end; i++) averagePrice += price.get(i);
        averagePrice /= (end - start + 1);

        double lower = 0;
        for (int i = start; i <= end; i++) {
            lower += Math.pow(i - averageDay, 2);
        }
        double upper = 0;
        for (int i = start - 1; i < end; i++) {
            upper += (i + 1 - averageDay) * (price.get(i) - averagePrice);
        }

        double bOne = upper / lower;
        double bZero = averagePrice - bOne * averageDay;
        HtmlParser.output(stock + "," + start + "," + end + "\n" + ff(bOne) + "," +
                ff(bZero) + "\n");
    }

    public static void crawl() throws IOException {
        // crawl
        Document doc = Jsoup.connect("https://pd2-hw3.netdb.csie.ncku.edu.tw/").get();
        // parse
        Element nameTable = doc.selectFirst("body > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1)");
        Element valTable = doc.selectFirst("body > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2)");
        assert nameTable != null;
        assert valTable != null;
        int day = Integer.parseInt(doc.title().substring(3));
        // open
        File file = new File("data.csv");
        if (!file.exists()) file.createNewFile();
        BufferedReader reader = new BufferedReader(new FileReader(file));
        // check original
        ArrayList<String> content = new ArrayList<>();
        for (int i = 0; i < 31; i++) content.add(reader.readLine());
        // write
        content.set(0, nameTable.children().text().replaceAll(" ", ","));
        content.set(day, valTable.children().text().replaceAll(" ", ","));
        BufferedWriter writer = new BufferedWriter(new FileWriter(file));
        for (int i = 0; i < 31; i++) writer.write(content.get(i) + "\n");
        writer.close();
    }
}
