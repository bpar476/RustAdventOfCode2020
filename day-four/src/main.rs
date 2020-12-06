use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn between(query: i32, min: i32, max: i32) -> bool {
    query >= min && query <= max
}

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());

    let re = Regex::new(r"(\w{3}):(.*?)( |$)").unwrap();

    let mut valid = 0;
    let mut contains_cid = false;
    let mut num_fields = 0;
    let mut current_pasport_valid = true;
    for line in reader.lines() {
        let line = line.unwrap();

        if current_pasport_valid {
            for mtch in re.captures_iter(&line) {
                if !current_pasport_valid {
                    break;
                }

                let val = &mtch[2];

                current_pasport_valid &= match &mtch[1] {
                    "byr" => {
                        let byr = val.parse::<i32>().unwrap();
                        between(byr, 1920, 2002)
                    }
                    "iyr" => {
                        let iyr = val.parse::<i32>().unwrap();
                        between(iyr, 2010, 2020)
                    }
                    "eyr" => {
                        let eyr = val.parse::<i32>().unwrap();
                        between(eyr, 2020, 2030)
                    }
                    "hgt" => {
                        let hgt_regex = Regex::new(r"(\d*)(cm|in)").unwrap();
                        match hgt_regex.captures(val) {
                            Some(cap) => {
                                if &cap[2] == "cm" {
                                    between(cap[1].parse::<i32>().unwrap(), 150, 193)
                                } else if &cap[2] == "in" {
                                    between(cap[1].parse::<i32>().unwrap(), 59, 76)
                                } else {
                                    false
                                }
                            }
                            None => false,
                        }
                    }
                    "hcl" => {
                        let hcl_regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
                        match hcl_regex.captures(val) {
                            Some(_) => true,
                            None => false,
                        }
                    }
                    "ecl" => match val {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false,
                    },
                    "pid" => {
                        let pid_regex = Regex::new(r"^\d{9}$").unwrap();
                        match pid_regex.captures(val) {
                            Some(_) => true,
                            None => false,
                        }
                    }
                    "cid" => true,
                    val => panic!("{}", val),
                };
                num_fields += 1;
                if mtch[1] == *"cid" {
                    contains_cid = true;
                }
            }
        }

        if line == "" {
            if (num_fields == 8 || (num_fields == 7 && !contains_cid)) && current_pasport_valid {
                valid += 1;
            }
            contains_cid = false;
            num_fields = 0;
            current_pasport_valid = true;
        }
    }

    // Need to do one last check because the last line of input is not a new line
    if num_fields == 8 || (num_fields == 7 && !contains_cid) && current_pasport_valid {
        valid += 1;
    }

    println!("{} valid passports", valid);
}
