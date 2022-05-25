# hwpx_table_parse
* hwpx 한글 파일 내에 존재하는 여러 표들의 벡터
```rust
pub struct HwpxDocs {
    tables: Vec<Table>,
}
```
* 각 표는 (행, 열) tuple을 key로 하고 셀 내용 String을 값으로 갖는 HashMap
```rust
pub struct Table {
    size: (usize, usize),
    cells: HashMap<CellAddr, String>,
}

struct CellAddr(usize, usize); // (row, column)
```
* 메소드
  * `HwpxDocs::new()`
  * `HwpxDocs::num_of_tables()`
  * `HwpxDocs::get_table()`
  * `Table::new()`
  * `Table::get_size()`
  * `Table::get_content()`
