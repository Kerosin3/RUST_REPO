use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Device {
    devname: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct ServerResponse {
    devname: String,
    info: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..=2 {
        let req = Device {
            devname: "smart_socket_#0".to_string(),
        };
        let res = reqwest::Client::new()
            .post("http://localhost:8080/getdevproperty")
            .json(&req)
            .send()
            .await?
            .json::<ServerResponse>()
            .await?;
        println!("{res:?}");
        sleep(Duration::from_millis(70)).await;
    }
    for _ in 0..=2 {
        let req = Device {
            devname: "termometer_#0".to_string(),
        };
        let res = reqwest::Client::new()
            .post("http://localhost:8080/getdevproperty")
            .json(&req)
            .send()
            .await?
            .json::<ServerResponse>()
            .await?;
        println!("{res:?}");
        sleep(Duration::from_millis(70)).await;
    }

    let req = Device {
        devname: "smart_socket_#1".to_string(),
    };
    let res = reqwest::Client::new()
        .post("http://localhost:8080/getdevproperty")
        .json(&req)
        .send()
        .await?
        .json::<ServerResponse>()
        .await?;
    println!("{res:?} -> NOT FOUND OK");
    let res = reqwest::Client::new()
        .get("http://localhost:8080/device")
        .query(&[("devname", "smart_socket_#0"), ("status", "on")])
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 200_u16);
    println!("sending query OK (assertion passed)");
    let res = reqwest::Client::new()
        .get("http://localhost:8080/device")
        .query(&[("devname", "NOT_EXISTING_DEVICE"), ("status", "on")])
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 404_u16);
    println!("sending query OK (assertion passed)");
    let res = reqwest::Client::new()
        .get("http://localhost:8080/device")
        .query(&[("devname", "NOT_EXISTING_DEVICE"), ("status", "OFF")])
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 404_u16);
    println!("sending query OK (assertion passed)");

    Ok(())
}
