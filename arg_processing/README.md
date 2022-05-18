# Rust Argument 처리 예제
* 2022.4.08 chan10 작성
* `Cargo.toml` - clap 사용
```toml
[dependencies]
clap = { version = "^3", features = ["derive"] }
```
* 도움말
```
about 내용(도움말 설명)

USAGE:
    arg_processing [OPTIONS]

OPTIONS:
    -h, --help                           도움말
    -f, --param-flag                     boolean을 인수로 받음(flag)
    -i, --param-int <PARAM_INT>          u64를 인수로 받음 [default: 7]
    -s, --param-string <PARAM_STRING>    String을 인수로 받음 [default: "default string"]
```
