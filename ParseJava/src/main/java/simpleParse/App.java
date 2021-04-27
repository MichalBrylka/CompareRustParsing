package simpleParse;

import java.util.Iterator;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public final class App {
    private App() {
    }

    public static void main(String[] args) {
        long startTime = System.currentTimeMillis();

        long sum = 0L;
        long iterations = Long.parseLong(args[0]);

        if (args.length > 2 && args[2].equals("--stream")) {
            Pattern pattern = Pattern.compile("\\|");
            for (int i = 0; i < iterations; i++)
                sum = SumLinePattern(args[1], pattern);
        } else {
            for (int i = 0; i < iterations; i++)
                sum = SumLine(args[1]);
        }

        System.out.println(sum);
        System.out.println("Java parsing took: " + (System.currentTimeMillis() - startTime) + " ms");

        // Get the Java runtime
        Runtime runtime = Runtime.getRuntime();
        long memory = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("Used memory is: " + bytesToMegabytes(memory) + " MiB");

        runtime.gc();
        memory = runtime.totalMemory() - runtime.freeMemory();
        System.out.println("Used memory is: " + bytesToMegabytes(memory) + " MiB");
    }

    private static final long MEGABYTE = 1024L * 1024L;

    static long bytesToMegabytes(long bytes) {
        return bytes / MEGABYTE;
    }

    static long SumLine(String text) {
        long result = 0L;
        String[] split = text.split("\\|");
        for (String v : split) {
            result += Long.parseLong(v.trim());
        }
        return result;
    }

    static long SumLinePattern(String text, Pattern pattern) {
        long result = 0L;
        Iterator<String> iter = pattern.splitAsStream(text).iterator();

        while (iter.hasNext()) {
            result += Long.parseLong(iter.next().trim());
        }
        return result;
    }
}