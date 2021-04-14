package trash;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Reader;
import java.util.HashMap;
import java.util.Map;
import com.opencsv.CSVWriter;

/**
 * Frequency analysis Inspired by
 * https://en.wikipedia.org/wiki/Frequency_analysis
 *
 * @author kasper
 */
// 01. Project structure is ridiculous. New project

// Observed execution time on netbeans: 135 ms
// Observed execution time on vscode: 102 ms

// 02. Changed file path so it's dynamic instead of hardcoded. Please don't
// hardcode paths, people...
// 03. Seems like the LinkedHashMap is a rawtype. My compiler hates it. Changing
// Must be synchronized apparently
// https://docs.oracle.com/javase/8/docs/api/java/util/LinkedHashMap.html
// 04. Splitting up functionalities to clean things up
// 05. Removing unnecessary file definitions
// 06. For loop and removing reverse iteration with collect at the end
// 07. Switch to integers cuz longs are flippin huge.
// 08. Restructured tallyChars and deleted printTally entirely
// 09. Renamed some stuff to make it more self explanatory
// 10. Wrapped Filereader in a buffered reader. Much faster.
// Program is now like... 67ms.
// So, I've come to find out, that the file might be running differently in
// vscode. Not quite certain.
// This will execute the code as well:
// > java .\src\trash\Trash.java
// ClassLoader tries to look for files in AppData for some reason.
// PROJECT_ROOT_DIRECTORY is not beautiful, but it works.
// Alas. I have sadly only gained 2400 ms when iterating 100 times. 
// That's 6000~ ms on the old file, vs. 3600~ ms on this one. = (

public class Trash {

    private static final String FILE_NAME = "FoundationSeries.txt";
    private static final String PROJECT_ROOT_DIRECTORY = new File("meep").getAbsolutePath().replace("meep", "");
    private static final String RESOURCES_DIRECTORY = PROJECT_ROOT_DIRECTORY + "/src/resources/";
    private static final String CSV_OUT = RESOURCES_DIRECTORY + "data.csv";

    private static BufferedReader getFileFromResources(String fileName) throws FileNotFoundException {
        return new BufferedReader(new FileReader(RESOURCES_DIRECTORY + fileName));
    }

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
        bWriter.write("");
        bWriter.close();
    }

    public static void main(String[] args) throws FileNotFoundException, IOException {
        long totalTime = 0;
        cleanDataFile(CSV_OUT);
        for(int i=0; i<100; i++){
            long startTime = System.nanoTime();
            Map<Character, Integer> mapOfCharacters = createMapOfCharacters(getFileFromResources(FILE_NAME));
            printCharactersFromMap(mapOfCharacters);
            long endTime = System.nanoTime();
            long finalTime = (endTime - startTime) / 1000000;
            totalTime=totalTime+finalTime;
            appendTimeToCsvFile(CSV_OUT, finalTime);
        }
        System.out.println(totalTime+ "ms");
    }

    private static Map<Character, Integer> createMapOfCharacters(Reader reader) throws IOException {
        Map<Character, Integer> mapOfCharacters = new HashMap<>();
        int b;
        while ((b = reader.read()) != -1) {
            Character letter = Character.toUpperCase((char) b);
            if (letter >= 'A' && letter <= 'Z')
                if (mapOfCharacters.containsKey(letter)) {
                    mapOfCharacters.put(letter, mapOfCharacters.get(letter) + 1);
                } else {
                    mapOfCharacters.put(letter, 0);
                }
        }
        return mapOfCharacters;
    }

    private static void printCharactersFromMap(Map<Character, Integer> mapOfCharacters) {
        mapOfCharacters.keySet().forEach(x -> System.out.println(x + ": " + mapOfCharacters.get(x).toString()));
    }
}
