// 2022.4.21 by chan10

use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

// 숫자를 1000자리마다 콤마로 구분되는 String으로 변환
pub trait CommaSeparatedNums {
    fn to_comma_string(&self) -> String;
}

/// 콤마로 구분된 String을 숫자로 변환
pub trait Num<T: FromStr> {
    fn from_comma_string(&self) -> Result<T, <T as FromStr>::Err>;
}

impl<T: Add<Output = T> + Copy + Display> CommaSeparatedNums for T {
    fn to_comma_string(&self) -> String {
        let mut negative = false; // 음수인지 판단
        let mut fractional = false; // 소수점 있는지 판단
        let mut int_part: Vec<char> = vec![]; // 정수부
        let mut frac_part: Vec<char> = vec![]; // 소수부

        for ch in self.to_string().as_str().chars() {
            // 숫자-> String -> &str -> 각 문자 iterator
            match ch {
                '-' => negative = true,
                '.' => fractional = true,
                _ => {
                    if fractional {
                        frac_part.push(ch);
                    } else {
                        int_part.push(ch);
                    }
                }
            }
        }

        let mut target = String::new(); // 최종 콤마로 구분된 문자열
        if negative {
            target.push('-')
        };

        let mut int_part_rev: Vec<char> = vec![]; // 정수부 역순
        for (i, ch) in int_part.iter().rev().enumerate() {
            if i != 0 && i % 3 == 0 {
                int_part_rev.push(',');
            }
            int_part_rev.push(*ch);
        }

        target.push_str(&int_part_rev.iter().rev().collect::<String>());

        if fractional {
            target.push('.');
            target.push_str(&frac_part.into_iter().collect::<String>());
        }

        target
    }
}

impl<T: FromStr> Num<T> for str {
    fn from_comma_string(&self) -> Result<T, <T as FromStr>::Err> {
        self.split(',')
            .collect::<Vec<&str>>()
            .concat()
            .as_str()
            .parse::<T>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{CommaSeparatedNums, Num};

    #[test]
    fn num_to_string() {
        assert_eq!((9489309).to_comma_string(), "9,489,309".to_string());
        assert_eq!((-9489309).to_comma_string(), "-9,489,309".to_string());
        assert_eq!(
            (9489309.2345).to_comma_string(),
            "9,489,309.2345".to_string()
        );
        assert_eq!(
            (-9489309.2345).to_comma_string(),
            "-9,489,309.2345".to_string()
        );
        assert_eq!((0.998765).to_comma_string(), "0.998765".to_string());
    }

    #[test]
    fn string_to_num() {
        assert_eq!(
            Ok(9489309 as i32),
            Num::<i32>::from_comma_string("9,489,309")
        );
        assert_eq!(
            Ok(23345.6789 as f32),
            Num::<f32>::from_comma_string("23,345.6789")
        );
        assert_eq!(
            Ok(-23345.6789 as f32),
            Num::<f32>::from_comma_string("-23,345.6789")
        );
    }
}
