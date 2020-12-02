mod day_01;
mod day_02;

fn main() {

    let result_01 = day_01::day_01(3, String::from("inputs/input_day_01"), 2020);
    let result_02 = day_02::day_02(String::from("inputs/input_day_02"));

    println!("day 1: {}", result_01);
    println!("day 2: {}", result_02);
}
