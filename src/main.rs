pub mod data_types;

use data_types::data_types::{ControlRequest, ControlType, ListShockersBaseResponse, Shock};
use dotenv::dotenv;
use reqwest::{header, Client};
use std::error::Error;

async fn post_control_request(
    client: &Client,
    api_url: &str,
    id: String,
    control_type: ControlType,
) -> Result<reqwest::Response, reqwest::Error> {
    let control_request = serde_json::to_string(&ControlRequest {
        shocks: vec![Shock {
            id: id,
            control_type: control_type,
            intensity: 1,
            duration: 300,
            exclusive: true,
        }],
        custom_name: "rusty".to_string(),
    })
    .unwrap();

    let resp = client
        .post(format!("{api_url}/2/shockers/control"))
        .body(control_request)
        .send()
        .await?;
    resp.error_for_status()
}

async fn get_shockers_own(
    client: &Client,
    api_url: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let resp = client
        .get(format!("{api_url}/1/shockers/own"))
        .send()
        .await?;
    resp.error_for_status()
}

fn handle_err<T: Error>(err: T) {
    println!("Error: {}", err)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let openshock_token = dotenv::var("OPENSHOCK_TOKEN").expect("missing OPENSHOCK_TOKEN");
    let api_url = "https://api.shocklink.net";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-type",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "accept",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "OpenShockToken",
        header::HeaderValue::from_str(&openshock_token).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let resp = get_shockers_own(&client, api_url).await;
    match &resp {
        Ok(_) => {}
        Err(err) => {
            handle_err(err);
        }
    }
    let shocker_list = if !&resp.is_err() {
        let shocker_list_response: ListShockersBaseResponse =
            serde_json::from_str(&resp.unwrap().text().await?.as_str())
                .expect("Data should be able to decoded");
        shocker_list_response.data
    } else {
        None
    }.unwrap();

    let resp = post_control_request(
        &client,
        api_url,
        shocker_list[0].shockers[0].id.to_string(),
        ControlType::Sound,
    )
    .await;
    match resp {
        Ok(_) => {}
        Err(err) => handle_err(err),
    }

    Ok(())
}
