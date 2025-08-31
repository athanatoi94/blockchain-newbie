use reqwest::Client;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiError {
    #[error("HTTP请求错误: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API返回错误: {0} - {1}")]
    ApiError(u16, String),
}

async fn make_open_sea_request() -> Result<serde_json::Value, ApiError> {
    let client = Client::new();

    //TODO: ERC721 实现待学习，没有rust SDK，除非用js SDK?
    let request_body = json!({
        "parameters": {
            "orderType": 0
        }
    });

    let response = client
        .post("https://api.opensea.io/api/v2/orders/abstract/seaport/listings")
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();

    if status.is_success() {
        let response_text = response.text().await?;
        let json_response: serde_json::Value = serde_json::from_str(&response_text)?;
        Ok(json_response)
    } else {
        let error_text = response.text().await?;
        Err(ApiError::ApiError(status.as_u16(), error_text))
    }
}

#[tokio::main]
async fn main() {
    match make_open_sea_request().await {
        Ok(response) => {
            println!("请求成功!");
            println!("{:#?}", response);
        }
        Err(e) => {
            eprintln!("请求失败: {}", e);
        }
    }
}