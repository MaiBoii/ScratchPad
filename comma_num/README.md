# comma_num
* 2022.4.21 chan10 작성
* 숫자를 1000자리마다 콤마(comma)로 구분된 문자열로 변환하는 트레잇

```rust
fn Num<T: FromStr>::from_comma_string(&self) -> Result<T, <T as FromStr>::Err>
```
* 콤마로 구분된 문자열 형태의 숫자를 숫자로 변환

```rust
fn CommaSeparatedNums::to_comma_string(&self) -> String
```
* 숫자를 콤마로 구분된 문자열로 변환
