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

    let mut input: Vec<Vec<char>> = Vec::new();
    for line in contents.split('\n') {
        let row: Vec<char> = line.chars().collect();
        if row.len() == 0 {
            continue;
        }
        input.push(row);
    }

    // println! {"PRINTING SOURCE"}
    // for row in input {
    //     for c in row {
    //         print!("{c}");
    //     }
    //     println!("")
    // }
    // println!("Finished printing");
    // return;

    let kernel1: Vec<Vec<char>> = [['X', 'M', 'A', 'S'].to_vec()].to_vec();
    let kernel2: Vec<Vec<char>> = [
        ['X', '.', '.', '.'].to_vec(),
        ['.', 'M', '.', '.'].to_vec(),
        ['.', '.', 'A', '.'].to_vec(),
        ['.', '.', '.', 'S'].to_vec(),
    ]
    .to_vec();
    let kernel3: Vec<Vec<char>> = [
        ['X'].to_vec(),
        ['M'].to_vec(),
        ['A'].to_vec(),
        ['S'].to_vec(),
    ]
    .to_vec();
    let kernel4: Vec<Vec<char>> = [
        ['.', '.', '.', 'X'].to_vec(),
        ['.', '.', 'M', '.'].to_vec(),
        ['.', 'A', '.', '.'].to_vec(),
        ['S', '.', '.', '.'].to_vec(),
    ]
    .to_vec();
    let kernel5: Vec<Vec<char>> = [['S', 'A', 'M', 'X'].to_vec()].to_vec();
    let kernel6: Vec<Vec<char>> = [
        ['S', '.', '.', '.'].to_vec(),
        ['.', 'A', '.', '.'].to_vec(),
        ['.', '.', 'M', '.'].to_vec(),
        ['.', '.', '.', 'X'].to_vec(),
    ]
    .to_vec();
    let kernel7: Vec<Vec<char>> = [
        ['S'].to_vec(),
        ['A'].to_vec(),
        ['M'].to_vec(),
        ['X'].to_vec(),
    ]
    .to_vec();
    let kernel8: Vec<Vec<char>> = [
        ['.', '.', '.', 'S'].to_vec(),
        ['.', '.', 'A', '.'].to_vec(),
        ['.', 'M', '.', '.'].to_vec(),
        ['X', '.', '.', '.'].to_vec(),
    ]
    .to_vec();

    let mut result: usize = 0;
    result += convolution_equality_count(&input, &kernel1);
    result += convolution_equality_count(&input, &kernel2);
    result += convolution_equality_count(&input, &kernel3);
    result += convolution_equality_count(&input, &kernel4);
    result += convolution_equality_count(&input, &kernel5);
    result += convolution_equality_count(&input, &kernel6);
    result += convolution_equality_count(&input, &kernel7);
    result += convolution_equality_count(&input, &kernel8);
    println!("Result = {result}");
}

fn part_two(contents: String) {
    //! Runs **part two** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.
    println!("In part 2");

    let mut input: Vec<Vec<char>> = Vec::new();
    for line in contents.split('\n') {
        let row: Vec<char> = line.chars().collect();
        if row.len() == 0 {
            continue;
        }
        input.push(row);
    }

    let kernel1: Vec<Vec<char>> = [
        ['M', '.', 'M'].to_vec(),
        ['.', 'A', '.'].to_vec(),
        ['S', '.', 'S'].to_vec(),
    ]
    .to_vec();
    let kernel2: Vec<Vec<char>> = [
        ['M', '.', 'S'].to_vec(),
        ['.', 'A', '.'].to_vec(),
        ['M', '.', 'S'].to_vec(),
    ]
    .to_vec();
    let kernel3: Vec<Vec<char>> = [
        ['S', '.', 'M'].to_vec(),
        ['.', 'A', '.'].to_vec(),
        ['S', '.', 'M'].to_vec(),
    ]
    .to_vec();
    let kernel4: Vec<Vec<char>> = [
        ['S', '.', 'S'].to_vec(),
        ['.', 'A', '.'].to_vec(),
        ['M', '.', 'M'].to_vec(),
    ]
    .to_vec();

    let mut result: usize = 0;

    result += convolution_equality_count(&input, &kernel1);
    result += convolution_equality_count(&input, &kernel2);
    result += convolution_equality_count(&input, &kernel3);
    result += convolution_equality_count(&input, &kernel4);
    println!("result part 2: {result}");
}

struct MSize {
    height: usize,
    width: usize,
}

fn convolution_equality_count(source: &Vec<Vec<char>>, kernel: &Vec<Vec<char>>) -> usize {
    let s_size: MSize = MSize {
        height: source.len(),
        width: source[0].len(),
    };
    let k_size: MSize = MSize {
        height: kernel.len(),
        width: kernel[0].len(),
    };
    // let (sh, sw) = (s_size.height, s_size.width);
    // let (h, w) = (k_size.height, k_size.width);
    // let (dh, dw) = (sh - h, sw - w);
    // println!("kernel size: width {w}, height {h}, width deltas {dw}, {dh}: source {sw}, {sh}");

    let mut cumulative: usize = 0;
    for dx in 0..=(s_size.width - k_size.width) {
        'outer: for dy in 0..=(s_size.height - k_size.height) {
            // Convolve your kernel
            for x in 0..(k_size.width) {
                'inner: for y in 0..(k_size.height) {
                    if kernel[y][x] == '.' {
                        continue 'inner;
                    }
                    if kernel[y][x] != source[y + dy][x + dx] {
                        continue 'outer;
                    }
                }
            }
            cumulative += 1;
            // println!("Hit at {dx}, {dy}");
        }
    }

    return cumulative;
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
