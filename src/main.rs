use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut expenses:Vec<i32> = Vec::new();

    let filename = "src/input";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let expense = line; // Ignore errors.
        expenses.push(expense.unwrap().parse::<i32>().unwrap());
    }

    let mut expense1;
    let mut expense2;

    loop {
        expense1 = expenses.pop().unwrap();
        expense2 = expenses.iter().find(|e| *e + expense1 == 2020);
        if expense2.is_some() { break; }
    }

    println!("{} {}", expense1, expense2.unwrap());

    let result = expense1 * expense2.unwrap();
    println!("{}", result);
}
