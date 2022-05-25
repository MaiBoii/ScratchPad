// 2022.5.04 by chan10
// hwpx 파일 안의 표들의 벡터
// 각 표는 (행, 열) tuple을 key로 하고 셀 내용 String을 값으로 갖는 HashMap

use quick_xml::events::{attributes::Attributes, Event};
use quick_xml::Reader;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fs, io, str};

const HWPX_EXT: &str = "hwpx";
const HWPX_XML: &str = "Contents/section0.xml";

// 지정된 디렉토리 밑의 hwpx 파일들의 목록들
pub fn hwpx_files(dir_name: &str) -> io::Result<Vec<PathBuf>> {
    let mut files_vec = Vec::new();

    for path in fs::read_dir(dir_name)? {
        let path = path?.path();
        if let Some(HWPX_EXT) = path.extension().and_then(OsStr::to_str) {
            files_vec.push(path.to_owned());
        }
    }
    Ok(files_vec)
}

// hwpx 파일 해체
pub fn hwpx_unzip(hwpx_path: &Path) -> io::Result<&Path> {
    use zip::ZipArchive;

    // hwpx 파일 존재 체크
    let dir = hwpx_path.file_stem(); // 확장자 제외하고 파일명만 추출
    let mut archive = ZipArchive::new(fs::File::open(hwpx_path)?)?;

    // 디렉토리 생성하여 압축해제
    let target_path = Path::new(dir.unwrap());
    archive.extract(target_path)?;

    Ok(target_path)
}

// hwpx 해체된 디렉토리 전체 삭제
pub fn del_hwpx_dir(dir: &Path) -> io::Result<()> {
    fs::remove_dir_all(dir)
}

#[derive(Debug)]
// 문서 전체: Vector of Tables
pub struct HwpxDocs {
    tables: Vec<Table>,
}

#[derive(Default, Debug)]
// 표 한개
pub struct Table {
    size: (usize, usize),
    cells: HashMap<CellAddr, String>,
}

#[derive(Eq, Hash, PartialEq, Default, Debug)]
struct CellAddr(usize, usize); // (row, column)

impl HwpxDocs {
    pub fn new(xml_path: &Path) -> Result<Self, quick_xml::Error> {
        let mut r = Reader::from_file(xml_path.join(HWPX_XML))?;
        r.trim_text(true);

        let hwpx_tables = hwpx_parse(r);

        Ok(Self {
            tables: hwpx_tables,
        })
    }

    // Hwpx 파일 내 table들 개수
    pub fn num_of_tables(&self) -> usize {
        self.tables.len()
    }

    // Hwpx 파일 내 index번째(0번부터 시작)의 reference of table 반환
    // - index가 table 개수 이상이면 None
    pub fn get_table(&self, index: usize) -> Option<&Table> {
        self.tables.get(index)
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            size: (0, 0),
            cells: HashMap::new(),
        }
    }

    // table의 가로, 세로 셀 개수
    pub fn get_size(&self) -> (usize, usize) {
        (self.size.0, self.size.1)
    }

    // 주어진 (가로, 세로) -0부터 시작- 셀의 내용물
    // - 병합된 셀은 None
    pub fn get_content(&self, addr: (usize, usize)) -> Option<&String> {
        self.cells.get(&CellAddr(addr.0, addr.1))
    }
}

fn parse_table_size(attrs: Attributes) -> (usize, usize) {
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    for attr in attrs {
        let item = attr.unwrap();
        let attr_value = |v| {
            str::from_utf8(v)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap()
        };

        match item.key {
            b"rowCnt" => {
                rows = attr_value(&item.value);
            }
            b"colCnt" => {
                cols = attr_value(&item.value);
            }
            _ => {}
        }
    }

    (rows, cols)
}

fn hwpx_parse(mut r: Reader<io::BufReader<fs::File>>) -> Vec<Table> {
    let mut hwpx_doc: Vec<Table> = Vec::new();

    // buffers for reader
    let mut buf: Vec<u8> = Vec::new();

    loop {
        // loop root
        match r.read_event(&mut buf) {
            // match level 0
            Ok(Event::Start(ref e)) if e.local_name() == b"tbl" => {
                let mut table = Table::new(); // 새로운 struct table 한개 만들고
                table.size = parse_table_size(e.attributes()); // size 파악

                let mut nested1: Vec<u8> = Vec::new();
                loop {
                    // loop <tbl>
                    nested1.clear();
                    match r.read_event(&mut nested1) {
                        // match level 1
                        Ok(Event::Start(ref e)) if e.local_name() == b"tc" => {
                            let mut nested2: Vec<u8> = Vec::new();
                            let mut text_lines: Vec<String> = Vec::new();
                            let mut col_addr: usize = 0;
                            let mut row_addr: usize = 0;

                            loop {
                                // loop <tc>
                                nested2.clear();
                                match r.read_event(&mut nested2) {
                                    // match level 2
                                    Ok(Event::Start(ref e)) if e.name() == b"hp:t" => {
                                        text_lines.push(
                                            r.read_text(b"hp:t", &mut Vec::new())
                                                .expect("디코딩 불가"),
                                        );
                                    }
                                    // match level 2
                                    Ok(Event::Empty(ref e)) if e.local_name() == b"cellAddr" => {
                                        for attr in e.attributes() {
                                            let x = attr.unwrap();
                                            match x.key {
                                                b"colAddr" => {
                                                    col_addr = str::from_utf8(&x.value)
                                                        .unwrap()
                                                        .parse()
                                                        .unwrap()
                                                }
                                                b"rowAddr" => {
                                                    row_addr = str::from_utf8(&x.value)
                                                        .unwrap()
                                                        .parse()
                                                        .unwrap()
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    Ok(Event::End(ref e)) if e.local_name() == b"tc" => {
                                        table.cells.insert(
                                            CellAddr(row_addr, col_addr),
                                            text_lines.join("\n"),
                                        );
                                        break;
                                    }
                                    // match level 2
                                    _ => {}
                                }
                            } // end of loop <tc>
                        }
                        // match level 1
                        Ok(Event::End(ref e)) if e.local_name() == b"tbl" => {
                            // </tbl>이면
                            hwpx_doc.push(table);
                            break;
                        }
                        // match level
                        _ => {}
                    }
                } // end of loop <tbl>
            }
            // match level 0
            Ok(Event::Eof) => break,
            // match level 0
            _ => {}
        }
        buf.clear();
    } // end of loop root

    // return Vec<Table>
    hwpx_doc
}
