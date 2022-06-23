use lazy_static::lazy_static;
use regex::Regex;

fn only_hangul(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"[\x{ac00}-\x{d7a3}]+").unwrap();    // [가-힣]
    }
    REGEX.find_iter(text).map(|m| m.as_str()).collect()
}

fn main() {
    let text: &str = "123bbasfsdf23asd2021-06-17";
    let re = Regex::new(r"\d{3}").unwrap();
    // Regex::find_iter() : 매치되는 부분(겹치지 않도록)에 대한 시작과 끝 바이트 인덱스들의 iterator 반환
    for cap in re.find_iter(text) {
        println!("{:?} {:?}", cap, cap.as_str());
    }

    let text = "Not my favorite movie: 'Citizen Kane' (1941).";
    // ' + ('외 아무거나) + ' + 공백 + ( + 네자리숫자 + )
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let caps = re.captures(text).unwrap();
    println!("{:?}", caps.get(0).unwrap().as_str()); // &caps[0] - 매치 전체
    println!("{:?}", caps.get(1).unwrap().as_str()); // &caps[1] - 매치 내에서 괄호로 표현된 첫번째 그룹
    println!("{:?}", caps.get(2).unwrap().as_str()); // &caps[2] - 매치 내에서 괄호로 표현된 두번째 그룹

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let caps = re.captures(text).unwrap();
    println!("{:?}", caps.name("title").unwrap().as_str()); // &caps["title"] - 매치 내에서 title로 이름붙은 그룹
    println!("{:?}", caps.name("year").unwrap().as_str()); // &caps["year"] - 매치 내에서 year로 이름붙은 그룹

    let text = "Not 한글hangul 영어2022 혼combined합";
    let v = only_hangul(text);
    println!("{:?}", v);

    let text = "가나 다라[ ○★   ]";
    // (한글 한글자 이상) + [ + 공백있거나 + (공백아닌 글자 하나이상) + 공백있거나 + ]
    let re = Regex::new(r"([\x{ac00}-\x{d7a3}\s]+)\[\s*\S+\s*\]").unwrap();
    let v: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    let caps = re.captures(text).unwrap();
    println!("{:?} {:?}", v, caps.get(1).unwrap().as_str());

    let text = "마바사 [   ]";
    // (한글 한글자 이상) + [ + (공백하나이상) + ]
    let re = Regex::new(r"([\x{ac00}-\x{d7a3}\s]+)\[\s+\]").unwrap();
    let v: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    let caps = re.captures(text).unwrap();
    println!("{:?} {:?}", v, caps.get(1).unwrap().as_str().trim());

    let text = "{ test_name }";
    let re = Regex::new(r"\s*\{\s*(\S+)\s*\}\s*").unwrap();
    let caps = re.captures(text).unwrap();
    println!("{}", caps.get(1).map_or("", |m| m.as_str()));
}
