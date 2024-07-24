// src/models.rs
use serde::{Deserialize, Serialize}; // Ensure both are imported
use std::fmt::Debug; // Import the Debug trait

#[derive(Deserialize, Serialize, Debug)] // Ensure Debug and Serialize are derived
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

#[derive(Debug, Deserialize, Serialize)] // Add Serialize here
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)] // Ensure Debug and Serialize are derived
pub struct NewsResponse {
    pub status: String,
    pub total_results: Option<u32>,
    pub articles: Vec<Article>, // Make this field public
}
