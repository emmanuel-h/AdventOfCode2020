mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {

    let result_01 = day_01::day_01(3, String::from("inputs/input_day_01"), 2020);
    let result_02 = day_02::day_02(String::from("inputs/input_day_02"));
    let result_03 = day_03::day_03(String::from("inputs/input_day_03"));
    let result_04 = day_04::day_04(String::from("inputs/input_day_04"));

    println!("day 1: {}", result_01);
    println!("day 2: {}", result_02);
    println!("day 3: {}", result_03);
    println!("day 4: {}", result_04);
}
