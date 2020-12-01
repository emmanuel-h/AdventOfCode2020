use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day_01() {
    let mut expenses: Vec<i32> = Vec::new();

    let filename = "inputs/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let expense = line;
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
