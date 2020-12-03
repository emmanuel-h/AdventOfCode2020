use std::fs::File;
use std::io::{BufRead, BufReader};

const COLUMNS:usize = 31;
const ROWS:usize = 323;

pub(crate) fn day_03(filename: String) -> i32 {
    let mut grid = [[0 as char; COLUMNS] ; ROWS];

    parse_file(&mut grid, filename);

    count_trees(&mut grid, 1, 1) *
        count_trees(&mut grid, 3, 1) *
        count_trees(&mut grid, 5, 1) *
        count_trees(&mut grid, 7, 1) *
        count_trees(&mut grid, 1, 2)
}

fn count_trees(grid: &mut [[char; 31]; 323], slope_column: usize, slope_row: usize) -> i32 {
    let mut actual_row = 0;
    let mut actual_column = 0;
    let mut trees = 0;

    while actual_row + slope_row < ROWS {
        actual_row = actual_row + slope_row;
        actual_column = (actual_column + slope_column) % COLUMNS;
        if grid[actual_row][actual_column] == '#' { trees = trees + 1 }
    }

    trees
}

fn parse_file(grid: &mut [[char; COLUMNS];ROWS], filename: String) {
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (line_index, line) in reader.lines().enumerate() {
        for (column_index, symbol) in line.unwrap().chars().enumerate() {
            grid[line_index][column_index] = symbol;
        }
    }
}
