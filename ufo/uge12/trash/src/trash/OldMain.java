package trash;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Reader;
import java.util.Collections;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.Map;
import static java.util.stream.Collectors.toMap;

/**
 * Frequency analysis Inspired by
 * https://en.wikipedia.org/wiki/Frequency_analysis
 *
 * @author kasper
 */
public class OldMain {

    private static final String FILE_NAME = "FoundationSeries.txt";
    private static final String PROJECT_ROOT_DIRECTORY = new File("meep").getAbsolutePath().replace("meep", "");
    private static final String RESOURCES_DIRECTORY = PROJECT_ROOT_DIRECTORY + "/src/resources/";
    private static final String CSV_OUT = RESOURCES_DIRECTORY + "old_data.csv";

    private static void appendTimeToCsvFile(String fileName, long time) throws IOException {
        FileWriter writer = new FileWriter(fileName, true);
        BufferedWriter bWriter = new BufferedWriter(writer);
        bWriter.write(String.valueOf(time));
        bWriter.newLine();
        bWriter.close();
    }

    private static void cleanDataFile(String fileName) throws IOException {
        FileWriter writer = new FileWriter(fileName, false);
        BufferedWriter bWriter = new BufferedWriter(writer);
        bWriter.write("execution_time");
        bWriter.newLine();
        bWriter.close();
    }

    public static void main(String[] args) throws FileNotFoundException, IOException {
        long totalTime = 0;
        cleanDataFile(CSV_OUT);
        for (int i = 0; i < 100; i++) {
            long startTime = System.nanoTime();

            Reader reader = new FileReader(RESOURCES_DIRECTORY+FILE_NAME);
            Map<Integer, Long> freq = new HashMap<>();
            tallyChars(reader, freq);
            print_tally(freq);

            long endTime = System.nanoTime();
            long finalTime = (endTime - startTime) / 1000000;
            totalTime = totalTime + finalTime;
            appendTimeToCsvFile(CSV_OUT, finalTime);
        }

        System.out.println(totalTime + "ms");
    }

    private static void tallyChars(Reader reader, Map<Integer, Long> freq) throws IOException {
        int b;
        while ((b = reader.read()) != -1) {
            try {
                freq.put(b, freq.get(b) + 1);
            } catch (NullPointerException np) {
                freq.put(b, 1L);
            }
            ;
        }
    }

    private static void print_tally(Map<Integer, Long> freq) {
        int dist = 'a' - 'A';
        Map<Character, Long> upperAndlower = new LinkedHashMap();
        for (Character c = 'A'; c <= 'Z'; c++) {
            upperAndlower.put(c, freq.getOrDefault(c, 0L) + freq.getOrDefault(c + dist, 0L));
        }
        Map<Character, Long> sorted = upperAndlower.entrySet().stream()
                .sorted(Collections.reverseOrder(Map.Entry.comparingByValue()))
                .collect(toMap(Map.Entry::getKey, Map.Entry::getValue, (e1, e2) -> e2, LinkedHashMap::new));
        for (Character c : sorted.keySet()) {
            System.out.println("" + c + ": " + sorted.get(c));
            ;
        }
    }
}
