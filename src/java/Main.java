// Equiv to "use std::io"
import java.util.Scanner;

// In Java, code must be inside a class.
// The main entry point is "public static void main(String[] args)".
public class Main {
    // "static final" defines a constant in Java.
    // "int" is a 32-bit integer type (signed, can be negative).
    // These constants define the game parameters.
    static final int LOWEST = 1;
    static final int HIGHEST = 100;
    static final int MAX_ATTEMPTS = 7;

    // The main entry point of the program.
    public static void main(String[] args) {
        // Scanner is used for reading user input from System.in (equivalent to stdin in Rust).
        Scanner scanner = new Scanner(System.in);

        // System.out.println() prints to the screen; equivalent to println!() macro in Rust.
        System.out.println("Think of a number between " + LOWEST + " and " + HIGHEST + ".");
        System.out.println("I will guess it within " + MAX_ATTEMPTS + " tries!");
        System.out.println("Ready? Press ENTER.");
        // See "waitForEnter()" method.
        waitForEnter(scanner);

        // Variables in Java can be modified without special keywords.
        int lowestGuess = LOWEST;
        int highestGuess = HIGHEST;

        // In Java, we can use Integer (wrapper class) to represent a nullable integer.
        // null is equivalent to None in Rust's Option<u32>.
        Integer answer = null;
        int attempts = 0;

        while (attempts < MAX_ATTEMPTS) {
            // clearscreen is not part of the standard Java library.
            // Other programming languages usually have something like Console.Clear() for this.
            // You should ignore this.

            // If the lowest number that the program remembers
            // is greater than the highest number that the program remembers,
            // then the user is lying / trying to cheat.
            if (lowestGuess > highestGuess) {
                System.out.println("ERR! According to you, the lowest number is " + lowestGuess + ", but the highest number is " + highestGuess + ".");
                System.out.println("Are you trying to cheat?");
                System.exit(1); // Terminates the program with code 1.
            }

            // If we were able to successfully guess the answer, then break out of this while-loop.
            if (answer != null) {
                break;
            }

            // See "predictNum()" method.
            int predictedNum = predictNum(lowestGuess, highestGuess);
            System.out.println("Alright! Attempt " + (attempts + 1) + ".");
            if (attempts == MAX_ATTEMPTS - 1) {
                System.out.println("Final attempt!");
            }

            // Java uses + operator for string concatenation.
            System.out.println("I think it's the number " + predictedNum + "...");
            System.out.println("Is it...");
            System.out.println("Too low? (<)");
            System.out.println("Too high? (>)");
            System.out.println("Or am I right? (=)");
            System.out.println("Or type (q) to quit.");

            // Equiv. to "loop" in Rust, or "while (true)" in Java.
            while (true) {
                // Read a line from the user using Scanner.
                // This is equivalent to io::stdin().read_line() in Rust.
                String input = scanner.nextLine();

                // Trims out whitespace from the input.
                input = input.trim();

                // If input is "q", quit.
                if (input.equals("q")) {
                    System.exit(0);
                }

                // If input is "<", then store as lowest number and break.
                if (input.equals("<")) {
                    // We are adding 1 here, to prevent the predicted number
                    // from being chosen again during search.
                    lowestGuess = predictedNum + 1;
                    break;
                }

                // If input is ">", then store as highest number and break.
                if (input.equals(">")) {
                    // Same thing here, removing 1 to prevent number from being
                    // chosen again.
                    highestGuess = predictedNum - 1;
                    break;
                }

                // If input is "=", then save the number as the answer and break.
                if (input.equals("=")) {
                    answer = predictedNum;
                    break;
                }

                // If any other input is received, then print this line and repeat.
                System.out.println("Invalid option. Please re-select.");
            }

            // Add one attempt after successful input.
            attempts += 1;
        }

        // clearscreen is not part of the standard Java library.
        // Other programming languages usually have something like Console.Clear() for this.
        // You should ignore this.

        // If the variable "answer" is null, then the program lost.
        if (answer == null) {
            System.out.println("Oh well... I'll get you next time!");
            System.exit(0); // Terminate program.
        }

        // Otherwise, if it has a value, print it.
        System.out.println("The number you were thinking of was " + answer + "!");
        System.out.println("I'm the best!");
        // Program ends here.

        // Close the scanner to free resources.
        scanner.close();
    }

    // This method attempts to guess the number using the formula:
    // (lowestNumber + highestNumber) / 2
    // This is Binary Search, and is more efficient than randomly guessing.
    // If the lowest number is equal to the highest number, then return the lowest number always,
    // no calculation necessary.
    static int predictNum(int lowest, int highest) {
        if (lowest == highest) {
            return lowest;
        }
        return (lowest + highest) / 2;
    }

    // This method queries a line from the user / terminal, then immediately discards it.
    // Since it pauses execution until ENTER / RETURN is pressed, this method
    // essentially functions as a "wait until user input" method.
    static void waitForEnter(Scanner scanner) {
        // Read and discard a line from the user.
        // Equivalent to io::stdin().read_line() in Rust.
        scanner.nextLine();
    }
}
