use core::num;
use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs;
use std::process;
use std::usize;

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

#[derive(Clone, Copy)]
struct Rule {
    first: usize,
    second: usize,
}

struct RuleBook {
    rules: Vec<Rule>,
}

impl RuleBook {
    fn new() -> RuleBook {
        //! Create a new empty rulebook
        RuleBook { rules: Vec::new() }
    }

    fn add_rule(&mut self, first: usize, second: usize) {
        //! Add a rule to the rulebook
        let new_rule = Rule { first, second };
        self.rules.push(new_rule);
    }

    fn follows_rules(&self, v1: usize, v2: usize) -> bool {
        for r in 0..self.rules.len() {
            if let Some(rule) = self.rules.get(r) {
                if rule.first == v2 && rule.second == v1 {
                    let (rf, rs) = (rule.first, rule.second);
                    // println!("Breaking rule {rf} and {rs} by values {v1} and {v2}");
                    return false;
                }
            }
        }
        return true;
    }

    fn check_set(&self, set: &Vec<usize>) -> bool {
        //! returns true of _set_ complies with
        //! the rulebook.
        for i in 0..set.len() {
            for j in (i + 1)..set.len() {
                let (v1, v2) = (set[i], set[j]);
                if !self.follows_rules(v1, v2) {
                    return false;
                }
            }
        }
        println! {"success"};
        return true;
    }

    fn order(&self, mut set: Vec<usize>) -> Vec<usize> {
        let mut applicable_rules: Vec<Rule> = Vec::new();
        for r in 0..self.rules.len() {
            let rule = self.rules[r];
            if set.contains(&rule.first) || set.contains(&rule.first) {
                applicable_rules.push(rule);
            }
        }

        for perm in set.iter().permutations(set.len()).unique() {
            let mut retval = Vec::new();
            for v in perm {
                retval.push(v.clone());
            }

            if self.check_set(&retval) {
                return retval;
            }
        }

        return Vec::new();
    }
    fn generate_order2(&self, mut num_set: Vec<usize>) -> Vec<usize> {
        let number_count = num_set.len();
        let mut result: Vec<usize> = Vec::new();
        let num_rules = self.rules.len();
        let mut passed_rules: usize = 0;
        let mut i = 0;

        let mut applicable_rules: Vec<Rule> = self.rules.clone();
        for i in applicable_rules.len()..0 {
            println! {"{i}"};
            if let Some(rule) = applicable_rules.get(i) {
                let (v1, v2) = (rule.first, rule.second);
                if !(num_set.contains(&v1) && num_set.contains(&v2)) {
                    applicable_rules.remove(i);
                }
            }
        }
        'outer: loop {
            if result.len() == number_count {
                break;
            }
            let mut viable_numbers = num_set.clone();
            for rule in self.rules.as_slice() {
                let (v1, v2) = (rule.first, rule.second);
                if viable_numbers.contains(&v2) && viable_numbers.contains(&v1) {
                    let index = viable_numbers.iter().position(|x| *x == v2).unwrap();
                    viable_numbers.remove(index);
                }
            }
            if viable_numbers.len() == 0 {
                eprint!("HELP!");
                process::exit(1);
            } else {
                println!("yay");
            }
            for v in viable_numbers {
                result.push(v);
                let index = num_set.iter().position(|x| *x == v).unwrap();
                num_set.remove(index);
            }
        }

        return result;
    }

    fn generate_order(&self) -> Vec<usize> {
        //! Generate a *one or multiple* lists of **orders**
        //! certain pages have to be in, depending on the
        //! rules which have been provided before

        let mut result: Vec<usize> = Vec::new();
        // Get a list of all numbers
        for rule in self.rules.as_slice() {
            let (v1, v2): (usize, usize) = (rule.first, rule.second);
            if !result.contains(&v1) {
                result.push(v1);
            }
            if !result.contains(&v2) {
                result.push(v2);
            }
        }

        let num_rules = self.rules.len();
        let mut passed_rules: usize = 0;
        let mut i = 0;
        // Change the list until it complies
        loop {
            if passed_rules >= num_rules {
                break;
            }
            let &rule = self
                .rules
                .get(i)
                .expect("Size checking should not allow for an index out of bounce");

            // Check if rule is passed
            let (v1, v2) = (rule.first, rule.second);
            let v1_pos: usize = result.iter().position(|x| *x == v1).unwrap();
            let v2_pos: usize = result.iter().position(|x| *x == v2).unwrap();

            if v2_pos < v1_pos {
                passed_rules = 0;
                result.swap(v1_pos, v2_pos);
            } else {
                passed_rules += 1;
            }
            i = (i + 1) % num_rules;
        }

        // merge rules

        return result;
    }
}

fn part_one(contents: String) {
    //! Runs **part one** of this weeks assigment
    //!
    //! Takes the input string as an argument and
    //! should print out the result.
    println!("In part 1");

    let mut iter = contents.split('\n').into_iter();
    if !iter.next().unwrap().is_empty() {
        eprintln!("Data does not have the expected starting blank line");
        process::exit(1);
    }

    let mut rules = RuleBook::new();
    // Collect the rules
    loop {
        let v = iter.next().unwrap_or_else(|| "");
        if v.is_empty() {
            println!("-= Finished collecting the rules =-");
            break;
        }
        let vals: Vec<&str> = v.split("|").collect();
        rules.add_rule(
            vals.get(0).unwrap().parse::<usize>().unwrap(),
            vals.get(1).unwrap().parse::<usize>().unwrap(),
        );
    }

    let mut page_sets: Vec<Vec<usize>> = Vec::new();
    // Collect the page sets
    loop {
        let v = iter.next().unwrap_or_else(|| "");
        if v.is_empty() {
            println!("-= Finished collecting the page sets =-");
            break;
        }
        let mut pages: Vec<usize> = Vec::new();
        for page in v.split(',') {
            pages.push(page.parse::<usize>().unwrap());
        }
        page_sets.push(pages);
    }

    let mut result: usize = 0;
    for ordering in page_sets {
        if rules.check_set(&ordering) {
            let addition = ordering[(ordering.len() - 1) / 2];
            result += addition;
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

    let mut iter = contents.split('\n').into_iter();
    if !iter.next().unwrap().is_empty() {
        eprintln!("Data does not have the expected starting blank line");
        process::exit(1);
    }

    let mut rules = RuleBook::new();
    // Collect the rules
    loop {
        let v = iter.next().unwrap_or_else(|| "");
        if v.is_empty() {
            println!("-= Finished collecting the rules =-");
            break;
        }
        let vals: Vec<&str> = v.split("|").collect();
        rules.add_rule(
            vals.get(0).unwrap().parse::<usize>().unwrap(),
            vals.get(1).unwrap().parse::<usize>().unwrap(),
        );
    }

    let mut page_sets: Vec<Vec<usize>> = Vec::new();
    // Collect the page sets
    loop {
        let v = iter.next().unwrap_or_else(|| "");
        if v.is_empty() {
            println!("-= Finished collecting the page sets =-");
            break;
        }
        let mut pages: Vec<usize> = Vec::new();
        for page in v.split(',') {
            pages.push(page.parse::<usize>().unwrap());
        }
        page_sets.push(pages);
    }

    let mut result: usize = 0;
    for ordering in page_sets {
        if !rules.check_set(&ordering) {
            let new_set: Vec<usize> = rules.generate_order2(ordering);

            result += new_set[(new_set.len() - 1) / 2];
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
