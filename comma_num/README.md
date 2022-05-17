# comma_num
* 2022.4.21 박찬열 작성
* 숫자를 1000자리마다 콤마(comma)로 구분된 문자열로 변환하는 트레잇

```rust
fn Num<T: FromStr>::from_comma_string(&self) -> Result<T, <T as FromStr>::Err>
```
* 콤마로 구분된 문자열 형태의 숫자를 숫자로 변환
* 예제
  ```rust
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
  ```

```rust
fn CommaSeparatedNums::to_comma_string(&self) -> String
```
* 숫자를 콤마로 구분된 문자열로 변환
* 예제
  ```rust
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
  ```