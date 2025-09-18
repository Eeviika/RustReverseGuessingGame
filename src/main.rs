// Equiv to "import system.io"
use std::io;
// Equiv to "import java.process.exit" (probably not a real import)
use std::process::exit;

// "const" defines a constant, and its value must be defined in the code, or else it will error!
// It must also have a specified type. "u32" is an "unsigned 32-bit integer."
// u32 types can store a number from 0 to 4,294,967,295. Unsigned numbers cannot be negative.
// These are usually used when a number typically SHOULDN'T be negative.
const START_LOWEST: u32 = 1;
const START_HIGHEST: u32 = 100;
const MAX_ATTEMPTS: u32 = 7;

// In Rust, functions are defined with "fn." Because Rust is a systems-programming language,
// this code does not need to be in any OOP-data type.
// In Rust, the main entry point is "fn main()".
fn main() {
    // println!() is a MACRO that prints to the screen; behaves similarly to System.out.println().
    println!("Think of a number between {} and {}.", START_LOWEST, START_HIGHEST);
    println!("I will guess it within {} tries!", MAX_ATTEMPTS);
    println!("Ready? Press ENTER.");
    // See "fn wait_for_enter()".
    wait_for_enter();

    // "let" defines a variable.
    // "mut" allows a variable to be modified.
    let mut lowest= START_LOWEST;
    let mut highest = START_HIGHEST;

    // Option<> types are difficult to explain in comments.
    // Just know that an Option is essentially classified as:
    // "A variable that may or may not contain a value."
    // Think of it like "null" in Java.
    //
    // For more specifics:
    // See https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
    let mut answer: Option<u32> = None;
    let mut attempts= 0;

    while attempts < MAX_ATTEMPTS {
        // "clearscreen" is not part of the standard Rust library, and is a Crate (a user-generated library).
        // Other programming languages usually have something like Console.Clear() for this.
        // You should ignore this.
        clearscreen::clear().expect("Failed to clear screen");

        // If the lowest number that the program remembers
        // is greater than the highest number that the program remembers,
        // then the user is lying / trying to cheat.
        if lowest > highest {
            println!("ERR! According to you, the lowest number is {}, but the highest number is {}.", lowest, highest);
            println!("Are you trying to cheat?");
            exit(1); // Terminates the program with code 1.
        }

        // If we were able to successfully guess the answer, then break out of this while-loop.
        if answer.is_some() {
            break;
        }

        // See "fn predict_num()".
        let predicted_num = predict_num(lowest, highest);
        println!("Alright! Attempt {}.", attempts + 1);
        if attempts == MAX_ATTEMPTS - 1 {
            println!("Final attempt!")
        }

        // println! can accept up to any number of arguments*, which are slotted
        // into the string at each pair of brackets "{}".
        //
        // * println! can't take an unlimited amount of arguments.
        println!("I think it's the number {}...", predicted_num);
        println!("Is it...");
        println!("Too low? (<)");
        println!("Too high? (>)");
        println!("Or am I right? (=)");
        println!("Or type (q) to quit.");

        // Equiv. to "while true" in Java.
        loop {
            // "String::new()" creates a new String. It is similar to Strings in Java.
            // The specifics aren't too important for this project or function.
            // Just know that this String stores the user's input.
            let mut input = String::new();

            // Equiv to "Scanner(System.in).nextLine()" in Java and storing it in "input"
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Trims out whitespace from the code.
            let input = input.trim();

            // If input is "q", quit.
            if input == "q" {
                exit(0);
            }

            // If input is "<", then store as lowest number and break.
            if input == "<" {
                // We are adding 1 here, to prevent the predicted number
                // from being chosen again during search.
                lowest = predicted_num + 1;
                break;
            }

            // If input is ">", then store as highest number and break.
            if input == ">" {
                // Same thing here, removing 1 to prevent number from being
                // chosen again.
                highest = predicted_num - 1;
                break;
            }

            // If input is "=", then save the number as the answer and break.
            if input == "=" {
                answer = Some(predicted_num);
                break;
            }

            // If any other input is received, then print this line and repeat.
            println!("Invalid option. Please re-select.")
        }

        // Add one attempt after successful input.
        attempts += 1;
    }

    // "clearscreen" is not part of the standard Rust library, and is a Crate (a user-generated library).
    // Other programming languages usually have something like Console.Clear() for this.
    // You should ignore this.
    clearscreen::clear().expect("Failed to clear screen");

    // If the variable "answer" has no value, then the program lost.
    if answer.is_none() {
        println!("Oh well... I'll get you next time!");
        exit(0); // Terminate program.
    }

    // Otherwise, if it does, grab that value and print it.
    println!("The number you were thinking of was {}!", answer.unwrap());
    println!("I'm the best!")
    // Program ends here.
}

// This function attempts to guess the number using the formula:
// (lowest_number + highest_number) / 2
// This is Binary Search, and is more efficient than randomly guessing.
// If the lowest number is equal to the highest number, then return the lowest number always,
// no calculation necessary.
fn predict_num(lowest: u32, highest: u32) -> u32 {
    if lowest == highest {
        return lowest;
    }
    (lowest + highest) / 2
}

// This function queries a line from the user / terminal, then immediately discards it.
// Since it pauses execution until ENTER / RETURN is pressed, this function
// essentially functions as a "wait until user input" function.
fn wait_for_enter() {
    // Variables that aren't read should start with an underscore.
    // "String::new()" creates a new String. It is similar to Strings in Java.
    // The specifics aren't too important for this project or function.
    let mut _enter_buffer = String::new();
    // Equiv to "Scanner(System.in).nextLine()" in Java and storing it in _enter_buffer
    io::stdin().read_line(&mut _enter_buffer).unwrap();
}