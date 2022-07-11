// AoC 2021 #2: https://adventofcode.com/2021/day/2
// 2022.7.08 by chan

// Part I:
// input 파일을 읽어 최종 horizontal position * depth 구하기
// 0에서 시작하여 "forward"이면 해당 값만큼 horizontal position에 더하기
// "down"이면 depth 더하고, "up"이면 depth에서 빼기

// Part II:
// 추가로 aim 추적
// "down X"이면 aim X만큼 증가
// "up X"이면 aim X만큼 감소
// "forward X"이면 horizontal position 증가 및 depth를 aim*X만큼 증가

use std::fs;
use std::io::Error;

const INPUT_FNAME: &str = "./input.txt";

fn main() -> Result<(), Error> {
    let input_str = fs::read_to_string(INPUT_FNAME)?;

    let position = &input_str
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0), |(h, d), x| {
            match (x.0, x.1.parse::<i32>().unwrap()) {
                ("forward", value) => (h + value, d),
                ("up", value) => (h, d - value),
                ("down", value) => (h, d + value),
                _ => (h, d),
            }
        });
    println!("AoC 2021 #2 part 1 : {}", position.0 * position.1);

    let position = &input_str
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0, 0), |(h, d, aim), x| {
            match (x.0, x.1.parse::<i32>().unwrap()) {
                ("forward", value) => (h + value, d + aim * value, aim),
                ("up", value) => (h, d, aim - value),
                ("down", value) => (h, d, aim + value),
                _ => (h, d, aim),
            }
        });
    println!("AoC 2021 #2 part 2 : {}", position.0 * position.1);

    Ok(())
}
