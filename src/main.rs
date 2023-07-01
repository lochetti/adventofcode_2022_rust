use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    match day_1() {
        Ok(calories) => print!("calories: {calories}"),
        Err(e) => println!("error {e:?}"),
    }
}

fn day_1() -> anyhow::Result<i32> {
    let file = File::open("../inputs/day1.input")?;

    let mut max_calories = i32::MIN;
    let mut calories_acc = 0;
    for line_res in io::BufReader::new(file).lines() {
        let Ok(line) = line_res else {
            break;
        };
        if line.is_empty() {
            if calories_acc > max_calories {
                max_calories = calories_acc;
            }
            calories_acc = 0;
        } else {
            calories_acc += line.parse::<i32>()?;
        }
    }

    Ok(max_calories)
}
