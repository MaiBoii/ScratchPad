// AoC #1: https://adventofcode.com/2021/day/1
// 2022.7.06 by chan

// Part I:
// input에 있는 일련의 값들이 바로 직전 값보다 증가된 횟수 구하기

// Part II:
// input에 있는 일련의 값들 세개씩 더해 다음 더해진 세개씩 보다 더 증가한 회수 구하기

use std::fs;
use std::io::Error;

const INPUT_FNAME: &str = "./input.txt";

fn main() -> Result<(), Error> {
    let input_str = fs::read_to_string(INPUT_FNAME)?;

    let increase_count = &input_str
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
        .windows(2)
        .filter(|item| item[0] < item[1])
        .count();
    println!("AoC 2021 #1 part 1 : {increase_count}");

    let window_increase_count = &input_str
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
        .windows(4)
        .filter(|item| item[0] < item[3])
        .count();
    println!("AoC 2021 #1 part 2 : {window_increase_count}");

    Ok(())
}
