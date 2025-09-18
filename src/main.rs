use std::io;
use std::process::exit;

const START_LOWEST: u32 = 1;
const START_HIGHEST: u32 = 100;
const MAX_ATTEMPTS: u32 = 7;

fn main() {
    println!("Think of a number between {} and {}.", START_LOWEST, START_HIGHEST);
    println!("I will guess it within {} tries!", MAX_ATTEMPTS);
    println!("Ready? Press ENTER.");
    wait_for_enter();

    let mut lowest: u32 = START_LOWEST;
    let mut highest: u32 = START_HIGHEST;
    let mut answer: Option<u32> = None;
    let mut attempts: u32 = 0;

    while attempts < MAX_ATTEMPTS {
        clearscreen::clear().expect("Failed to clear screen");

        if lowest > highest {
            println!("ERR! According to you, the lowest number is {}, but the highest number is {}.", lowest, highest);
            println!("Are you trying to cheat?");
            exit(1);
        }

        if answer.is_some() {
            break;
        }

        let predicted_num = predict_num(lowest, highest);
        println!("Alright! Attempt {}.", attempts + 1);
        if attempts == MAX_ATTEMPTS - 1 {
            println!("Final attempt!")
        }
        println!("I think it's the number {}...", predicted_num);
        println!("Is it...");
        println!("Too low? (<)");
        println!("Too high? (>)");
        println!("Or am I right? (=)");
        println!("Or type (q) to quit.");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let input = input.trim();

            if input == "q" {
                exit(0);
            }

            if input == "<" {
                lowest = predicted_num + 1;
                break;
            }

            if input == ">" {
                highest = predicted_num - 1;
                break;
            }

            if input == "=" {
                answer = Some(predicted_num);
                break;
            }

            println!("Invalid option. Please re-select.")
        }

        attempts += 1;
    }

    clearscreen::clear().expect("Failed to clear screen");

    if answer.is_none() {
        println!("Oh well... I'll get you next time!");
        exit(0);
    }

    println!("The number you were thinking of was {}!", answer.unwrap());
    println!("I'm the best!")
}

fn predict_num(lowest: u32, highest: u32) -> u32 {
    if lowest == highest {
        return lowest;
    }
    (lowest + highest) / 2
}
    
fn wait_for_enter() {
    let mut _enter_buffer = String::new();
    io::stdin().read_line(&mut _enter_buffer).unwrap();
}