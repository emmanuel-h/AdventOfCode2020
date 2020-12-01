use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub(crate) fn day_01() -> i32 {
    let mut expenses: Vec<i32> = Vec::new();

    let filename = "inputs/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let expense = line;
        expenses.push(expense.unwrap().parse::<i32>().unwrap());
    }

    expenses.into_iter()
        .combinations(3)
        .find(|t| t[0] + t[1] + t[2] == 2020)
        .map(|t| t[0] * t[1] * t[2])
        .unwrap()
}
