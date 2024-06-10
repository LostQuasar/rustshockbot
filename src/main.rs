pub mod data_types;

use data_types::data_types::{ControlRequest, ControlType, ListShockersBaseResponse, ListShockersResponse, Shock};
use dotenv::dotenv;
use reqwest::{header, Client};
use std::error::Error;

async fn post_control_request(
    client: &Client,
    api_url: &str,
    id: String,
    control_type: ControlType,
) -> Result<reqwest::Response, Box<dyn Error>> {
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
    Ok(resp.error_for_status()?)
}

async fn get_shockers_own(
    client: &Client,
    api_url: &str,
) ->  Result<Option<Vec<ListShockersResponse>>, Box<dyn Error>> {
    let resp = client.get(format!("{api_url}/1/shockers/own")).send().await;
    let list_shockers_response: ListShockersBaseResponse =
        serde_json::from_str(resp?.text().await?.as_str())?;
    Ok(list_shockers_response.data)
}

fn handle_err(err: &Box<dyn Error>) {
    println!("Error: {}", err)
}

#[tokio::main]
async fn main(){
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
        .build().expect("err");

    let resp = get_shockers_own(&client, api_url).await;
    match &resp {
        Ok(_) => {}
        Err(err) => {
            handle_err(err);
        }
    }

    let resp = post_control_request(
        &client,
        api_url,
        resp.unwrap().unwrap()[0].shockers[0].id.to_string(),
        ControlType::Sound,
    )
    .await;
    match resp {
        Ok(_) => {}
        Err(err) => handle_err(&err),
    }
}
