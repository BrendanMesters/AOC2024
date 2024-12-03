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

    'reports: for report in contents.lines() {
        // read in data
        let levels: Vec<usize>  = report.split_whitespace()
            .map(
                |v| 
                v.parse::<usize>().unwrap_or_else(
                    |_e| {
                        eprintln!("The input file was formatted incorrectly, it should only contain numbers seperated by whitespace characters"); 
                        process::exit(1)
                    }
                )
            ).collect();

        // calculate safe and unsafe reports
        let increasing: bool = levels[0] < levels[1];
        for i in 0..(levels.len() - 1) {
            let diff: usize = levels[i].abs_diff(levels[i + 1]);
            if (1 > diff || 3 < diff) ||  ((levels[i] > levels[i + 1]) == increasing) {
                continue 'reports;
            }
        }
        result += 1;
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

    'reports: for report in contents.lines() {
        // read in data
        let levels: Vec<usize>  = report.split_whitespace()
            .map(
                |v| 
                v.parse::<usize>().unwrap_or_else(
                    |_e| {
                        eprintln!("The input file was formatted incorrectly, it should only contain numbers seperated by whitespace characters"); 
                        process::exit(1)
                    }
                )
            ).collect();


        'deletion: for d in 0..=(levels.len()) {
            let mut lv: Vec<usize> = levels.clone();
            if d > 0 {
                lv.remove(d - 1);
            }
            for v in lv.clone(){
                print!("{v} ");
            }

            // checks this subset
            let increasing: bool = lv[0] < lv[1];
            for i in 0..(lv.len() - 1) {
                let diff: usize = lv[i].abs_diff(lv[i + 1]);
                if (1 > diff || 3 < diff) ||  ((lv[i] > lv[i + 1]) == increasing) {
                    // failure
                    println!("    failed at index {i}");
                    continue 'deletion;
                }
            }
            println!("    succeeded!");
            // succes
            result += 1;
            continue 'reports;
        }
        println!("This loop was a total failure");


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
