use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

fn main() {
    let (part, input_file): (usize, String) = parse_input();

    // Read the file
    let contents: String = fs::read_to_string(&input_file).unwrap_or_else(|_e| {
        eprintln!("File location \"{input_file}\" not found, please provide a valid file location");
        process::exit(1)
    });

    if part == 1 {
        part_one(contents);
    } else if part == 2 {
        part_two(contents);
    } else {
        println!("invalid part value was provided")
    }
}

fn part_one(_contents: String) {
    //! Runs **part one** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.

    let result: usize = 0;
    println!("result part 1: {result}")
}

fn part_two(_contents: String) {
    //! Runs **part two** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.

    let result: usize = 0;
    println!("result part 2: {result}");
}

fn parse_input() -> (usize, String) {
    //! Reads command line input
    //!
    //! This function reads the command line inputs, parses it
    //! for validity, and returns the 'part' and the 'input file'
    //! if they are provided correctly (at least those arguments)
    //! and 'part' should be castable to usize.
    // Parse the input
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: <program> <part (1 or 2)> <file>");
        process::exit(1);
    }

    let part = args[1].parse::<usize>().unwrap_or_else(|e| {
        eprintln!("Error, part value should be a valid integer");
        process::exit(1);
    });

    let file = args[2].clone();

    println!("Input is: \"{part}\"; \"{file}\"");
    (part, file)
}
