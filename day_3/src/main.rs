use regex::Regex;
use std::env;
use std::fs;
use std::process;

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

fn part_one(contents: String) {
    //! Runs **part one** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.
    println!("In part 1");

    let mut result: usize = 0;

    let finder: Regex = Regex::new(r"mul\((?<v1>\d{1,3}),(?<v2>\d{1,3})\)")
        .expect("hardcoded regex may never fail");
    let mut doing: bool = true;
    for line in contents.split("\n") {
        for mul in finder.captures_iter(line) {
            let v1 = mul
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Should only consists of digits, in line with the capture group");
            let v2 = mul
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Should only consists of digits, in line with the capture group");
            let mp = v1 * v2;
            println! {"found {v1}×{v2} = {mp}"};
            result += mp;
        }
    }

    println!("result part 1: {result}")
}

fn part_two(contents: String) {
    //! Runs **part two** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.
    println!("In part 2");

    let mut result: usize = 0;

    let finder: Regex = Regex::new(r"mul\((?<v1>\d{1,3}),(?<v2>\d{1,3})\)|don't\(\)|do\(\)")
        .expect("hardcoded regex may never fail");
    let mut doing: bool = true;
    for line in contents.split("\n") {
        for mul in finder.captures_iter(line) {
            let command = mul.get(0).unwrap().as_str();
            println!("found line: {command}");
            if command == "don't()" {
                doing = false;
                continue;
            }
            if command == "do()" {
                doing = true;
                continue;
            }
            if !doing {
                continue;
            }

            let v1 = mul
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Should only consists of digits, in line with the capture group");
            let v2 = mul
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("Should only consists of digits, in line with the capture group");
            let mp = v1 * v2;
            println! {"found {v1}×{v2} = {mp}"};
            result += mp;
        }
    }

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

    let part = args[1].parse::<usize>().unwrap_or_else(|_e| {
        eprintln!("Error, part value should be a valid integer");
        process::exit(1);
    });

    let file = args[2].clone();

    println!("Input is: \"{part}\"; \"{file}\"");
    (part, file)
}
