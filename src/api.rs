use std::error::Error;

use isahc::prelude::*;
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "http://manga-api.bytecats.codes/mk/search?q=";

#[derive(Debug, Deserialize, Serialize)]
pub struct Manga {
    pub id: String,
    pub title: String,
    pub author: String,
    #[serde(rename = "latestChapter")]
    pub latest_chapter: String,
    #[serde(rename = "lastUpdateDatetime")]
    pub last_update_datetime: String,
    #[serde(rename = "imagePreview")]
    pub image_preview: String,
    #[serde(rename = "viewCount")]
    pub view_count: String,
}

pub fn search_manga(query: &str) -> Result<Vec<Manga>, Box<dyn Error>> {
    let search_url = format!("{}{}", API_URL, query.trim());
    let response = isahc::get(search_url)?;
    let body = response.text()?;
    let manga_list: Vec<Manga> = serde_json::from_str(&body)?;
    Ok(manga_list)
}

