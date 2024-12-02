use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::zip;
use std::process;
use std::str::FromStr;

fn main() {
    let (part, input_file): (usize, String) = parse_input();

    // Read the file
    let unchecked_contents: Result<String, std::io::Error> = fs::read_to_string(&input_file);
    if unchecked_contents.is_err() {
        eprintln!("File location \"{input_file}\" not found, please provide a valid file location");
        process::exit(1);
    }
    let contents: String = unchecked_contents.expect("Was just checked to be OK");

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
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    for line in contents.lines() {
        let vals: Vec<&str> = line.split_whitespace().collect();
        let (v1, v2) = (
            i64::from_str(vals.get(0).expect("")).expect(""),
            i64::from_str(vals.get(1).expect("")).expect(""),
        );
        left_list.push(v1);
        right_list.push(v2);
        // println!("v1 = {v1}; v2 = {v2}");
    }
    left_list.sort();
    right_list.sort();

    let mut cumulative_value: u64 = 0;
    for (left, right) in zip(left_list, right_list) {
        let difference = i64::abs(left - right);
        cumulative_value += difference as u64;
    }
    println!("result part 1: {cumulative_value}")
}

fn part_two(contents: String) {
    //! Runs **part two** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    for line in contents.lines() {
        let vals: Vec<&str> = line.split_whitespace().collect();
        let (v1, v2) = (
            i64::from_str(vals.get(0).expect("")).expect(""),
            i64::from_str(vals.get(1).expect("")).expect(""),
        );
        left_list.push(v1);
        right_list.push(v2);
        // println!("v1 = {v1}; v2 = {v2}");
    }

    let mut acum_vals: usize = 0;
    for lv in left_list {
        let right_count: usize = right_list
            .clone()
            .into_iter()
            .filter(|value| *value == lv)
            .count();
        acum_vals += right_count * lv as usize;
    }
    println!("{acum_vals}");

    return;
    // trying to do it fancily

    // count ID occurances
    let mut left_map: HashMap<i64, usize> = HashMap::new();
    let mut right_map: HashMap<i64, usize> = HashMap::new();
    for v in left_list {
        left_map
            .entry(v)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        // *left_map.entry(v).or_default() += 1;
    }
    for v in right_list {
        right_map
            .entry(v)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        // *right_map.entry(v).or_default() += 1;
    }
    for k in right_map.keys() {
        let v = right_map.get(k).unwrap();
        println!("{k}; {v}");
    }

    let mut cumulative_value: usize = 0;
    let right_keys: Vec<&i64> = right_map.keys().collect();

    for key in left_map.keys() {
        if right_keys.contains(&key) {
            let (lv, rv) = (left_map.get(key).unwrap(), left_map.get(key).unwrap());
            let addition: usize = lv * rv * (*key as usize);
            cumulative_value += addition;
            println!("key = {key}; lv, rv = ({lv}, {rv}); addition = {addition}")
        }
    }
    println!("result part 2: {cumulative_value}");
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
