use prost_types::Timestamp;
use std::io::{stdin};
extern crate hex_slice;
use hex_slice::AsHex;
use testproto::{testproto_client::TestprotoClient, NameRequest, PersonResponse};
pub mod testproto {
    tonic::include_proto!("testproto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TestprotoClient::connect("http://[::1]:8080").await?;
    loop {
        println!("\n-----Enter a name------");
        let mut name_input = String::new();

        stdin().read_line(&mut name_input).unwrap();
        let name_input = name_input.trim();
        let request = tonic::Request::new(NameRequest {
            name: name_input.to_owned(),
        });
        let response = client.enquire_person(request).await?;
        let resp: WrapPerson = WrapPerson::convert(response.into_inner());
        println!("---> Server answered {}", resp);
    }
    Ok(())
}
struct WrapPerson {
    timestamp: Timestamp,
    name: String,
    hash: Vec<u8>,
}
impl WrapPerson {
    fn convert(pr: PersonResponse) -> Self {
        Self {
            name: pr.confirmation,
            timestamp: pr.timestamp.unwrap(),
            hash: pr.hash,
        }
    }
}

impl std::fmt::Debug for WrapPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersonResponse")
            .field("name", &self.name)
            .field("timestamp", &self.timestamp)
            .field("hash", &format!("{:x?}", &self.hash))
            .finish()
    }
}
impl std::fmt::Display for WrapPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nrequested name:{}\nhash:{:02x}\ntimestamp:{}",
            self.name,
            self.hash.as_hex(),
            self.timestamp
        )
    }
}
