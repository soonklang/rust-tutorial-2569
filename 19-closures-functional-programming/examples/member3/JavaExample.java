import java.util.function.Function;

public class Main {
    public static void main(String[] args) {
        int n = 10;
        Function<Integer, Integer> addN = x -> x + n;

        System.out.println(addN.apply(5)); // 15
    }
}
