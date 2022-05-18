// 2022.4.08 by chan10

mod args; // 별도 파일로 존재하는 아규먼트 처리 모듈

fn main() {
    let arguments = args::arg_process();

    println!("{:?}", arguments.param_string);
    println!("{:?}", arguments.param_int);
    println!("{:?}", arguments.param_flag);
}
