#![allow(unused_imports)]
use prost_types::Timestamp;
//use std::time::SystemTime;
use blake2::{Blake2b512, Blake2s256, Digest};

use testproto::{
    testproto_server::{Testproto, TestprotoServer}, // import server
    NameRequest,
    PersonResponse,
};
use tonic::{transport::Server, Request, Response, Status};
use tracing::Level;

use tracing_subscriber::fmt;
pub mod testproto {
    tonic::include_proto!("testproto");
}
//main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let phone_service = RpcService::default();

    let subscriber = fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("start main server loop");
    Server::builder()
        .add_service(TestprotoServer::new(phone_service))
        .serve(address)
        .await?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct RpcService {}

#[tonic::async_trait]
impl Testproto for RpcService {
    // imp protocol!
    async fn enquire_person(
        &self,
        request: Request<NameRequest>,
    ) -> Result<Response<PersonResponse>, Status> {
        let r = request.into_inner();
        tracing::info!("got name >>{}<<", r.name);
        let mut hasher = blake2::Blake2b512::new();
        hasher.update(&r.name);
        let hash_itself = hasher.finalize();
        match r.name {
            ref name if name == "Alex" => Ok(Response::new(testproto::PersonResponse {
                confirmation: { "ALEX!".to_string() },
                timestamp: Some(std::time::SystemTime::now().into()),
                hash: hash_itself[..].to_owned(),
            })),
            _ => Ok(Response::new(testproto::PersonResponse {
                confirmation: { format!("you asked for a person with a name {} ", r.name) },
                timestamp: Some(std::time::SystemTime::now().into()),
                hash: hash_itself[..].to_owned(),
            })),
        }
    }
}
