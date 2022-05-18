// 아규먼트 처리 모듈 예
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about = "about 내용(도움말 설명)", long_about = None)]
pub struct Args {
    /// String을 인수로 받음
    #[clap(short='s', long, default_value_t = String::from("default string"))]
    pub param_string: String,

    /// u64를 인수로 받음
    #[clap(short = 'i', long, default_value_t = 7)]
    pub param_int: u64,

    /// boolean을 인수로 받음(flag)
    #[clap(short = 'f', long)]
    pub param_flag: bool,

    /// 도움말
    #[clap(short, long, display_order = 1)]
    pub help: bool,
}

pub fn arg_process() -> Args {
    Args::parse()
}
