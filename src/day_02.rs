use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

struct PasswordCheck {
    letter: char,
    min_occurrence: usize,
    max_occurrence: usize,
    password: String,
}

#[allow(dead_code)]
impl PasswordCheck {
    fn check_count(&self) -> bool {
        let count = self.password.chars()
            .filter(|c| c == &self.letter)
            .count();
        return count <= self.max_occurrence && count >= self.min_occurrence
    }

    fn check_position(&self) -> bool {
        (self.password.chars().nth(self.min_occurrence - 1).unwrap() == self.letter)
            ^
            (self.password.chars().nth(self.max_occurrence - 1).unwrap() == self.letter)
    }
}

pub(crate) fn day_02(filename: String) -> usize {
    let reg = Regex::new(r"^(\d{1,2})-(\d{1,2}) ([a-z]): ([a-z]{0,100})$").unwrap();
    let mut passwords_to_check: Vec<PasswordCheck> = Vec::new();

    parse_file(&mut passwords_to_check, filename, reg);

    passwords_to_check.iter()
        .filter(|p| p.check_position())
        .count()
}

fn parse_file(passwords_to_check: &mut Vec<PasswordCheck>, filename: String, reg: Regex) {
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for cap in reg.captures_iter(line.unwrap().as_ref()) {
            passwords_to_check.push(
                PasswordCheck {
                    letter: cap[3].parse().unwrap(),
                    min_occurrence: cap[1].parse().unwrap(),
                    max_occurrence: cap[2].parse().unwrap(),
                    password: cap[4].parse().unwrap(),
                }
            );
        }
    }
}
