
use serde::{Deserialize, Serialize}; 
use std::fmt::Debug; 

#[derive(Deserialize, Serialize, Debug)] 
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub urlToImage: Option<String>,
    pub publishedAt: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)] 
pub struct NewsResponse {
    pub status: String,
    pub total_results: Option<u32>,
    pub articles: Vec<Article>, 
}
