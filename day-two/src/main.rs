extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Policy {
    min: i32,
    max: i32,
    letter: String,
}

type PasswordData = Vec<(Policy, String)>;

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());

    let input: Vec<(Policy, String)> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| -> (Policy, String) {
            let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
            let captures = re.captures(&line).unwrap();
            let pol = Policy {
                min: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                max: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                letter: captures.get(3).unwrap().as_str().to_string(),
            };
            return (pol, captures.get(4).unwrap().as_str().to_string());
        })
        .collect();

    let test_input_raw = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let test_input = test_input_raw
        .iter()
        .map(|line| -> (Policy, String) {
            let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
            let captures = re.captures(&line).unwrap();
            let pol = Policy {
                min: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                max: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                letter: captures.get(3).unwrap().as_str().to_string(),
            };
            return (pol, captures.get(4).unwrap().as_str().to_string());
        })
        .collect();

    println!("{}", part_one(&test_input));
    println!("{}", part_two(&test_input));

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(password_data: &PasswordData) -> i32 {
    let mut num_valid_passwords = 0;
    for pair in password_data {
        let (pol, pass) = pair;
        let count = pass.chars().fold(0, |acc, c| -> i32 {
            if c.to_string() == pol.letter {
                return acc + 1;
            }
            acc
        });

        if count >= pol.min && count <= pol.max {
            num_valid_passwords += 1;
        }
    }
    num_valid_passwords
}

fn part_two(password_data: &PasswordData) -> i32 {
    let mut num_valid_passwords = 0;

    fn letter_at_position(chars: &Vec<char>, pos: i32) -> String {
        chars.get(pos as usize).unwrap().to_string()
    }

    for pair in password_data {
        let (pol, pass) = pair;

        let chars: Vec<char> = pass.chars().collect();

        let letter_pos_a = letter_at_position(&chars, pol.min - 1);
        let letter_pos_b = letter_at_position(&chars, pol.max - 1);
        if (letter_pos_a == pol.letter || letter_pos_b == pol.letter)
            && letter_pos_a != letter_pos_b
        {
            num_valid_passwords += 1;
        }
    }

    num_valid_passwords
}
