// use std::fs::File;
// use std::collections::HashMap;

use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};



fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

fn construct_body(method: &str) -> Value {
    let map = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": method,
        "params": []
    });

    map
}

#[derive(Debug)]
#[derive(Deserialize)]
struct GetRpc {
    result: String,
    id: String,
    error: Option<String>,
}

const STATUS: &'static str = "JSON";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:18332/")
    .headers(construct_headers())
    .basic_auth("ljs",Some("dlwotmd"))
    .json(&construct_body("getbestblockhash"))
    .send()
    .await?;

    match STATUS {
        "JSON" => {
            let modified_res: Value = res.json().await?;
            println!("{:#?}", modified_res);
        },
        "CUSTOM" => {
            let modified_res: GetRpc = res.json().await?;
            println!("{:#?}", modified_res);
        },
        "TEXT" => {
            let modified_res = res.text().await?;
            println!("{:#?}", modified_res);
        },
        _ => panic!()
    }






    Ok(())

    // let resp = reqwest::get("http://127.0.0.1:18332/")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);

    // let file = File::create("result.txt")?;
    // serde_json::to_writer(file, &resp)?;


}
