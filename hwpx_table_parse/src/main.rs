// 테스트용 main

mod hwpx;

use std::io;

fn main() -> io::Result<()> {
    let hwpx_dir = hwpx::hwpx_files("./");
    let hwpx_file_path = &hwpx_dir.unwrap()[0];
    let xml_path = hwpx::hwpx_unzip(hwpx_file_path)?;
    println!("{:?}", xml_path);
    let x = hwpx::HwpxDocs::new(xml_path).unwrap(); // vector of tables

    println!("전체 표 개수 = {}", x.num_of_tables());

    let table_index = 0;
    let table = x.get_table(table_index);
    
    let row = 3;
    let col = 0;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 2;
    let col = 0;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 2;
    let col = 1;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 4;
    let col = 1;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 4;
    let col = 3;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 6;
    let col = 4;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 7;
    let col = 4;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 9;
    let col = 5;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let row = 10;
    let col = 6;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    let table_index = 1;
    let table = x.get_table(table_index);
    
    let row = 1;
    let col = 2;
    println!(
        "{:?} 크기 {}번째 표 중 셀 ({}, {}) = {:?}",
        table.unwrap().get_size(),
        table_index,
        row,
        col,
        table.unwrap().get_content((row, col))
    );

    hwpx::del_hwpx_dir(xml_path)
}
