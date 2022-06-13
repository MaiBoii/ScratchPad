# PyO3 튜토리얼
* https://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/

## 환경 설정
* `mkdir tutorial`
* `cd tutorial`
* `python3 -m venv .venv`
* `source .venv/bin/activate`
* `pip install maturin`
* `maturin develop`

## 파이썬에서 Rust 함수 호출하기: 단순 곱하기
### rust 파일에서
* `pyo3` prelude 사용
* 함수에 `#[pyfunction]` 붙이기
* 함수 결과를 `PyResult`로 감싸기
* `#[pymodule]` 함수 추가

### `cargo.toml`에서
```toml
[dependencies]
pyo3 = { version = "0.16", features = ["extension-module"] }

[lib]
name = "rust"
crate-type = ["cdylib"]
```

### `multiply.py` 파일에서
```python
import rust
rust.multiply(2, 3)
```

### 실행
```bash
maturin develop --release
python tutorial.py
```

## 파이썬에서 Rust 함수 호출하기: 피보나치 수열
### 파이썬 파일에서
```python
import timeit

# 파이썬으로 구현한 피보나치 수열 : def get_fibonacci()

print("python fib(5): ", timeit.timeit("get_fibonacci(5)", setup='from __main__ import get_fibonacci'))
print("rust fib(5): ", timeit.timeit("get_fibonacci(5)", setup="from rust import get_fibonacci"))

print("rust fib(150): ", timeit.timeit("get_fibonacci(150)", setup='from __main__ import get_fibonacci'))
print("rust fib(150): ", timeit.timeit("get_fibonacci(150)", setup="from rust import get_fibonacci"))
```

### 실행
```bash
maturin develop --release
python fibonacci.py
maturin develop
python fibonacci.py
```

## 파이썬에서 Rust 함수 호출하기: 전달 인수 데이터형 변환
* Python과 Rust 타입 매핑 : https://pyo3.rs/main/conversions/tables.html

### 실행
```bash
maturin develop --release
python type_conv.py
```

## 파이썬에서 Rust 함수 호출하기: Rust struct 사용
* `#[pyclass]`: struct를 class로 인식하도록 알려줌
* `#[pymethods]`: impl 블록 내의 struct 메소드들을 파이썬 class 메소드로 인식하도록 알려줌
* `#[pyo3(get, set)]`: struct 내 항목들을 파이썬에서 get 또는 set하는지 알려줌
* `#[new]`: struct 생성자를 파이썬의 class 생성자로 인식하도록 알려줌
* 기존 `add_function()` 대신 `add_class::<>()`으로 등록

### 실행
```bash
maturin develop --release
python class_struct.py
```

## 파이썬에서 Rust 함수 호출하기: 복잡한 데이터를 rust에게 보내기
### `cargo.toml`에서
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 실행
```bash
pip install pydantic
maturin develop --release
python serde_ex.py
```

## 파이썬에서 Rust 함수 호출하기: 파이썬에서 정의된 logger를 rust에서 사용
### `cargo.toml`에서
```toml
pyo3-log = "0.6"
log = "0.4"
```

### rust 파일에서
* 등록할 때 `pyo3_log::init();` 포함

## 파이썬에서 Rust 함수 호출하기: 예외처리
* rust쪽 함수에서 에러 발생되고 파이썬에서 메시지 표출
