// 첫번째 예제
//-------------

// fn multiply(a: isize, b: isize) -> isize {
//     a * b
// }

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(dict_printer, m)?)?;
    m.add_function(wrap_pyfunction!(word_printer, m)?)?;
    m.add_class::<RustStruct>()?; // struct명 제공
    m.add_function(wrap_pyfunction!(human_says_hi, m)?)?;

    pyo3_log::init();
    m.add_wrapped(wrap_pyfunction!(log_example))?;
    m.add_wrapped(wrap_pyfunction!(log_different_levels))?;

    m.add_function(wrap_pyfunction!(greater_than_2, m)?)?;

    Ok(())
}

// 두번째 예제
//-------------

#[pyfunction]
fn get_fibonacci(number: isize) -> PyResult<u128> {
    if number == 1 {
        return Ok(1);
    } else if number == 2 {
        return Ok(2);
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    Ok(sum)
}

// 세번째 예제
//-------------
#[pyfunction]
// 파이썬에서 list[T] -> rust에서 Vec<T>
fn list_sum(a: Vec<isize>) -> PyResult<isize> {
    let mut sum: isize = 0;
    for i in a {
        sum += i;
    }
    Ok(sum)
}

use std::collections::HashMap;
#[pyfunction]
// 파이썬에서 dict[K, V] -> rust에서 HashMap<K, V>
fn dict_printer(hm: HashMap<String, String>) {
    for (key, value) in hm {
        println!("{} {}", key, value)
    }
}

#[pyfunction]
fn word_printer(mut word: String, n: isize, reverse: bool, uppercase: bool) {
    if reverse {
        let mut reversed_word = String::new();
        for c in word.chars().rev() {
            reversed_word.push(c);
        }
        word = reversed_word;
    }
    if uppercase {
        word = word.to_uppercase();
    }
    for _ in 0..n {
        println!("{}", word);
    }
}

// 네번째 예제
//-------------
#[pyclass] // struct 정의
pub struct RustStruct {
    #[pyo3(get, set)] // struct 항목들을 get하고 싶은지, set하고 싶은지
    pub data: String,
    #[pyo3(get, set)] // struct 항목들을 get하고 싶은지, set하고 싶은지
    pub vector: Vec<u8>,
}

#[pymethods] // impl 블럭
impl RustStruct {
    #[new] // 파이썬의 class로
    pub fn new(data: String, vector: Vec<u8>) -> RustStruct {
        RustStruct { data, vector }
    }

    pub fn printer(&self) {
        println!("{}", self.data);
        for i in &self.vector {
            println!("{}", i);
        }
    }

    pub fn extend_vector(&mut self, extension: Vec<u8>) {
        println!("{}", self.data);
        for i in extension {
            self.vector.push(i);
        }
    }
}

// 다섯번째 예제
//---------------
use serde::{Deserialize, Serialize};

#[pyfunction]
fn human_says_hi(human_data: String) {
    println!("{}", human_data);
    let human: Human = serde_json::from_str(&human_data).unwrap();

    println!(
        "struct 다루기:\n {:#?}.\n {}은 {}살입니다.",
        human, human.name, human.age,
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct Human {
    name: String,
    age: u8,
}

// 여섯번째 예제
//---------------
use log::{debug, error, info, warn};

#[pyfunction]
fn log_different_levels() {
    error!("logging an error");
    warn!("logging a warning");
    info!("logging an info message");
    debug!("logging a debug message");
}

#[pyfunction]
fn log_example() {
    info!("A log message from {}!", "Rust");
}

// 일곱번째 예제
//---------------
use std::fmt;

#[derive(Debug)] // custom MyError 정의
struct MyError {
    pub msg: &'static str, // custom 메시지 전달용
}

impl std::error::Error for MyError {} // Error trait 구현

// Display trait 구현
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rust로부터 Error 메시지: {}", self.msg)
    }
}

// From trait 구현
use pyo3::{exceptions::PyOSError, prelude::*};
impl std::convert::From<MyError> for PyErr {
    fn from(err: MyError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}

#[pyfunction] // 에러 발생
fn greater_than_2(number: isize) -> Result<isize, MyError> {
    if number <= 2 {
        Err(MyError {
            msg: "number is less than or equal to 2",
        })
    } else {
        Ok(number)
    }
}
