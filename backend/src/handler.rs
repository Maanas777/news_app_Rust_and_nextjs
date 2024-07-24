
use crate::model::{Article, NewsResponse};

use warp::{Rejection, Reply};
use dotenv::dotenv;
use std::env;


#[derive(Debug)]
struct CustomError {
    message: String,
}

impl warp::reject::Reject for CustomError {}
pub async fn fetch_news() -> Result<Vec<Article>, Rejection> {
    dotenv().ok();
    println!("fetch_news called");

    // Fetch API key
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("API_KEY not found in environment variables");
            return Err(warp::reject::custom(CustomError {
                message: "API_KEY not found".to_string(),
            }));
        }
    };
    println!("API key obtained: {}", api_key);

    
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);
    println!("Request URL: {}", url);

    
    let client = reqwest::Client::new();
    let response = match client.get(&url)
        .header("User-Agent", "my-app") // Set User-Agent header
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("Request failed: {:?}", e);
            return Err(warp::reject::custom(CustomError {
                message: format!("Request failed: {:?}", e),
            }));
        }
    };

   
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("Failed to read response body: {:?}", e);
            return Err(warp::reject::custom(CustomError {
                message: format!("Failed to read response body: {:?}", e),
            }));
        }
    };

    // println!("Raw response body: {}", body);


    let json_response = match serde_json::from_str::<NewsResponse>(&body) {
        Ok(res) => res,
        Err(e) => {
            println!("Failed to parse JSON: {:?}", e);
            return Err(warp::reject::custom(CustomError {
                message: format!("Failed to parse JSON: {:?}", e),
            }));
        }
    };

    println!("Response received: {:?}", json_response);
    Ok(json_response.articles)
}



pub async fn handle_news() -> Result<impl Reply, Rejection> {
 
    match fetch_news().await {
        Ok(articles) => {
            println!("Successfully fetched news articles");
            let json_response = warp::reply::json(&articles);
            // println!("Response: {}", json_response.into_response().body().as_ref().to_str().unwrap_or("Empty response"));
            Ok(json_response)
        }
        Err(e) => {
            println!("Failed to fetch news articles: {:?}", e);
            Err(e)
        }
    }
}
