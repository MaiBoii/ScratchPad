// AoC 2021 #3 : https://adventofcode.com/2021/day/3

// Part I:
// 이진 형태로 되어 있는 input 파일 읽어 power consumption 구하기
// 각 자리수별로 더 많이 출현한 1 또는 0의 값을 결정하여 이진수로 표현한 값을 십진수로 변환하면 gamma rate
// 각 자리수별로 덜 출현한 1 또는 0의 값을 결정하여 이진수로 얻고 십진수로 변환하면 epsilon rate
// power consumption = gamma rate * epsilon rate

// Part II:
// lie support rating = oxygen generator rating * CO2 scrubber rating 구하기
// 각 자리수별로 차례대로 더 많이 출현한 1 또는 0의 값을 결정하고, 덜 출현한 값을 가진 행들은 버림. 한 행만 남았을 때 십진수로 변환하면 oxygen generator rating value
// CO2 scrubber rating value는 반대로 덜 출현한 값을 결정하고, 더 많이 출현한 값을 가진 행들은 버림. 한 행만 남았을 때 십진수로 변환

use std::fs;
use std::io::Error;

const INPUT_FNAME: &str = "./input.txt";
const WIDTH: usize = 12;

fn main() -> Result<(), Error> {
    let input_str = fs::read_to_string(INPUT_FNAME)?;

    let counter_gamma = &input_str
        .lines()
        .fold([0_isize; WIDTH], |mut count_array, str_slice| {
            for (i, c) in str_slice.chars().enumerate() {
                if c == '1' {
                    count_array[i] += 1;
                } else {
                    count_array[i] -= 1;
                }
            }
            count_array
        })
        .into_iter()
        .map(|i| if i >= 0 { '1' } else { '0' })
        .collect::<String>();
    let counter_epsilon = counter_gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();
    let gamma = usize::from_str_radix(counter_gamma.as_str(), 2).unwrap();
    let epsilon = usize::from_str_radix(counter_epsilon.as_str(), 2).unwrap();
    println!("Advent of Code #3 part 1 : {}", gamma * epsilon);

    let mut input_vec_for_oxy: Vec<&str> = input_str.lines().collect();
    let mut input_vec_for_co2 = input_vec_for_oxy.clone();
    let mut popular_char: char;

    for i in 0..WIDTH {
        let count_1 = input_vec_for_oxy.iter().fold(0, |mut count, str_slice| {
            if str_slice.chars().nth(i).unwrap() == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            count
        });

        if count_1 >= 0 {
            popular_char = '1';
        } else {
            popular_char = '0';
        }
        input_vec_for_oxy = input_vec_for_oxy
            .into_iter()
            .filter(|&line| line.chars().nth(i).unwrap() == popular_char)
            .collect::<Vec<&str>>();
        if input_vec_for_oxy.len() == 1 {
            break;
        }
    }
    for i in 0..WIDTH {
        let count_1 = input_vec_for_co2.iter().fold(0, |mut count, str_slice| {
            if str_slice.chars().nth(i).unwrap() == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            count
        });

        if count_1 >= 0 {
            popular_char = '1';
        } else {
            popular_char = '0';
        }
        input_vec_for_co2 = input_vec_for_co2
            .into_iter()
            .filter(|&line| line.chars().nth(i).unwrap() != popular_char)
            .collect::<Vec<&str>>();
        if input_vec_for_co2.len() == 1 {
            break;
        }
    }

    let oxy = usize::from_str_radix(input_vec_for_oxy.pop().unwrap(), 2).unwrap();
    let co2 = usize::from_str_radix(input_vec_for_co2.pop().unwrap(), 2).unwrap();
    println!("Advent of Code #3 part 2 : {}", oxy * co2);

    Ok(())
}
