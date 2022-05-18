use bookstore::bookstore_server::{Bookstore, BookstoreServer};
use bookstore::{GetBookRequest, GetBookResponse};
use tonic::{transport::Server, Request, Response, Status};

mod bookstore {
    include!("bookstore.rs");
}

// 서비스를 위한 struct 정의
#[derive(Default)]
pub struct BookStoreImpl {}

// .proto에서 정의된 서비스를 위한 rpc 구현
#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    // 함수로 구현된 rpc
    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = GetBookResponse {
            id: request.into_inner().id,
            author: "Peter".to_owned(),
            name: "Zero to One".to_owned(),
            year: 2014,
        };

        // .proto에 정의된 SayResponse message
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 서비스를 위한 주소 정의
    let addr = "[::1]:50051".parse().unwrap();

    // 서비스 생성
    let bookstore = BookStoreImpl::default();
    println!("Bookstore Server listening on {}", addr);

    // 서버에 서비스 추가
    Server::builder()
        .add_service(BookstoreServer::new(bookstore))
        .serve(addr)
        .await?;

    Ok(())
}
