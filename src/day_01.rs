use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub(crate) fn day_01(combination_number: usize, filename: String, goal: i32) -> i32 {
    let mut expenses: Vec<i32> = Vec::new();

    parse_file(&mut expenses, filename);

    expenses.into_iter()
        .combinations(combination_number)
        .find(|t| t.into_iter().sum::<i32>() == goal)
        .unwrap()
        .iter()
        .product()
}

fn parse_file(expenses: &mut Vec<i32>, filename: String) {
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for expense in reader.lines() {
        expenses.push(expense.unwrap().parse::<i32>().unwrap());
    }
}
