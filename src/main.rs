// env module provides access to command-line arguments.
// fs module allows you to interact with the file system.
use std::env;
use std::fs;

fn main() {
    // The `env::args` function returns an iterator of the command line arguments that were used to run this program.
    // The first argument is always the path to the program itself, so we use `.skip(1)` to skip over it and collect the rest into a `Vec<String>`.
    // `Vec<String>` is a vector (a resizable array) that holds elements of type `String`.
    let args: Vec<String> = env::args().skip(1).collect();

    // let raw_args: Vec<String> = env::args().collect();

    // Check if the number of arguments provided by the user is exactly one. If not, the program prints an error message.
    // `args.len()` returns the number of elements in the vector, which we expect to be 1 for the file path.
    // If the condition is not met, we use `eprintln!` to print an error message to the standard error (stderr) stream,
    // and then exit the program with a status code of 1 to indicate an error using `std::process::exit(1)`.
    if args.len() != 1 {
        eprintln!("Usage: {} <file_path>", env::args().next().unwrap());
        std::process::exit(1);
    }

    // The `&String` is a reference to a `String` in the `args` vector, specifically the first user-provided argument which is the file path.
    // We use `&args[0]` to get a reference to the first element of the vector without taking ownership, allowing us to use `file_path` without moving the value out of `args`.
    let file_path: &String = &args[0];

    // `fs::read_to_string(file_path)` attempts to read the file at the path specified by `file_path` into a `String`.
    // If successful, it returns the file contents as a `String`. If it fails (e.g., if the file doesn't exist), the program will panic and print the specified error message.
    // `.expect("Failed to read the file")` is used here to handle the potential error. If an error occurs, it will stop the program and display the message.
    let contents: String = fs::read_to_string(file_path).expect("Failed to read the file");

    // The `contents.lines()` method splits the file contents into lines and returns an iterator over the lines.
    // `.count()` is then called on this iterator to count the number of elements (lines) it contains, effectively giving us the total number of lines in the file.
    let line_count = contents.lines().count();

    // Finally, we print the line count to the console using the `println!` macro, which prints formatted text to the standard output (stdout).
    println!("The file has {} lines.", line_count);
    // println!("Args in the vector: {:?}.", args);
    // println!("Args in the vector: {:?}.", raw_args);
}

// Questions

// What is a stream

// What is a standard error (stderr) stream

// Explain the difference parts of the following code:
// env::args().next().unwrap()
