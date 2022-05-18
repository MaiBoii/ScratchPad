# gRPC Server example
* https://betterprogramming.pub/building-a-grpc-server-with-rust-be2c52f0860e

## 필요성
* HTTP나 JSON이 web API 생성을 위해 널리 사용되는 방법
* 하지만 serialize 때문에 느림
  - streaming 어려움
  - client 라이브러리 만들기 어려움

## gRPC
* 데이터센터를 중심으로, 들어오고 나가는 통신을 효율적으로 수행하게 해줌
* HTTP2를 통해 양방향 streaming 가능
* 여러 언어에서 사용 가능
* 효율적인 client 라이브러리 생성 가능
* protocol buffer라는 바이너리 프로토콜을 사용하여 serialize 빠름

## Protocol Buffer
* 언어 중립적으로 data serializing하는 메커니즘
* 메시지와 서비스를 정의하기 위한 문법으로 predefined structure 가짐
  - 서비스는 함수들
  - 메시지는 함수들에게 전달하는 아규먼트
  - value는 함수로부터 리턴 받는 결과

### 문법
* protocol buffer에 두가지 정의
  - `service`
  - `message`
* `.proto` 확장자를 가진 파일에 반드시 두가지 정의되어야 함
  - `syntax`
  - `package`