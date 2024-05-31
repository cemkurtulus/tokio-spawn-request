use std::error::Error;
use std::fs::File;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let files = vec![0,1,2,3,4];
    let mut handles = Vec::new();

    for i in files{
        handles.push(tokio::spawn(async move {
            read_csv(i).await.expect("TODO: panic message");
        }));
    }

    for item in handles{
        item.await?;
    }

    Ok(())
}

async fn read_csv(file_number: i32) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}.csv", file_number);
    println!("{}", file_name);
    let file = File::open(file_name).expect("file_name yok");
    let mut rdr =
        csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", &record[0]);

        let request_model = RequestModel{
            opportunity_id: (&record[0]).parse().unwrap(),
        };

        send_request(request_model).await
    }

    Ok(())
}

#[derive(Serialize)]
pub struct RequestModel {
    pub opportunity_id: String
}

async fn send_request(model: RequestModel){
    let mut map: HeaderMap = HeaderMap::new();
    map.insert("Authorization", HeaderValue::from_str("Bearer cem").unwrap());

    let res = reqwest::Client::new()
        .post("https://rust.requestcatcher.com/")
        .headers(map)
        .json(&serde_json::json!(&model))
        .send()
        .await.expect("TODO: panic message");

    println!("{:?}", res.status())
}