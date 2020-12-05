use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }
}

pub(crate) fn day_04(filename: String) -> i32 {
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut valid_passports_count = 0;

    let mut passport = Passport::default();

    for line in reader.lines() {
        let line_unwrapped = line.unwrap();
        if line_unwrapped.is_empty() {
            println!("{:?}", passport);
            if passport.valid() { valid_passports_count = valid_passports_count + 1 };
            println!("{}", valid_passports_count);
            passport = Passport::default();
        } else {
            let parts = line_unwrapped.split_whitespace();
            for part in parts {
                match part.split_at(3).0 {
                    "byr" => passport.byr = true,
                    "iyr" => passport.iyr = true,
                    "eyr" => passport.eyr = true,
                    "hgt" => passport.hgt = true,
                    "hcl" => passport.hcl = true,
                    "ecl" => passport.ecl = true,
                    "pid" => passport.pid = true,
                    "cid" => passport.cid = true,
                    _ => eprintln!("{}", part)
                }
            }
        }
    }

    valid_passports_count
}
