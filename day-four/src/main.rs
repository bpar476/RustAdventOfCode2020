use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// struct PassportData {
//     byr: i32,
//     iyr: i32,
//     eyr: i32,
//     hgt: i32,
//     hcl: String,
//     ecl: String,
//     pid: i32,
//     cid: i32,
// }

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());

    let re = Regex::new(r"(\w{3}):(.*?)( |$)").unwrap();

    let mut valid = 0;
    let mut contains_cid = false;
    let mut num_fields = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for mtch in re.captures_iter(&line) {
            num_fields += 1;

            if mtch[1] == *"cid" {
                contains_cid = true;
            }
        }

        if line == "" {
            if num_fields == 8 || (num_fields == 7 && !contains_cid) {
                valid += 1;
            }
            contains_cid = false;
            num_fields = 0;
        }
    }

    // Need to do one last check because the last line of input is not a new line
    if num_fields == 8 || (num_fields == 7 && !contains_cid) {
        valid += 1;
    }

    println!("{} valid passports", valid);
}
