use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use reqwest::{Client};
use scraper::{Html, Selector};

extern crate chrono;
extern crate chrono_tz;

use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use_cache_or_fetch().await
}

async fn use_cache_or_fetch() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = "./cache";
    let cache_filename = format!("{}.txt", get_today());

    let body: String = match cache_check(&cache_dir, &cache_filename) {
        Ok(content) => content,
        _ => {
            let url = "https://www.takaratomy.co.jp/products/wixoss/event/search.html";

            let client: Client = Client::new();
            let res = client.get(url)
                .header(reqwest::header::COOKIE, "wixAge=conf")
                .send().await?;
            let body = res.text().await?;

            if save_to_cache(&cache_dir, &cache_filename, &body).await.is_ok() {
                println!("キャッシュに保存成功");
            } else {
                println!("キャッシュに保存失敗");
            }

            body
        }
    };

    let selector = "#detail_cont_inner_fukuoka .event_box";
    let document: Html = Html::parse_document(&body);
    let main_selector: Selector = Selector::parse(selector).unwrap();

    // let content = document.select(&main_selector).next().map(|element| element.inner_html()).unwrap();
    let events: Vec<EventDetail> = document.select(&main_selector).into_iter().map(|c| {
        parse_event(&c.inner_html())
    }).collect();

    for e in events {
        println!("{}\n", e);
    }

    Ok(())
}

fn ensure_cache_dir_exists(cache_dir: &str) -> std::io::Result<()> {
    fs::create_dir_all(cache_dir)
}

async fn save_to_cache(cache_dir: &str, cache_filename: &str, content: &str) -> std::io::Result<()> {
    ensure_cache_dir_exists(cache_dir).unwrap();
    let path = Path::new(cache_dir).join(cache_filename);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct EventDetail {
    name: String,
    shop_name: String,
    shop_link: String,
    time: String,
    format: String,
}

impl Display for EventDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}\n\t{}\n\t{}\n\t{}\n\t{}", self.name, self.shop_name, self.shop_link, self.time, self.format)
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

fn parse_event<'a>(src: &'a str) -> EventDetail {
    let html = Html::parse_document(src);
    let selector_event_name: Selector = Selector::parse("dd.event_name").unwrap();
    let selector_shop_name: Selector = Selector::parse("dd.shop_name a").unwrap();
    let selector_time: Selector = Selector::parse("dd.time").unwrap();
    let selector_format: Selector = Selector::parse("dd.event_name span").unwrap();

    let extract_text = |selector: &Selector| {
        let mut extracted_text = String::new();
        if let Some(element) = html.select(selector).next() {
            if let Some(first_child) = element.first_child() {
                if let Some(text) = first_child.value().as_text() {
                    extracted_text = text.to_string();
                }
            }
        }
        extracted_text
    };

    EventDetail {
        name: extract_text(&selector_event_name),
        shop_name: extract_text(&selector_shop_name),
        shop_link: html.select(&selector_shop_name).next().unwrap().value().attr("href").unwrap_or("").to_string(),
        time: extract_text(&selector_time),
        format: extract_text(&selector_format),
    }
}

fn cache_check(dir: &str, filename: &str) -> Result<String, std::io::Error> {
    let path: PathBuf = PathBuf::from(format!("{}/{}", dir, filename));
    if path.exists() {
        // println!("cache found");
        let mut file: File = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        println!("cache not found");
        Err(std::io::Error::new(std::io::ErrorKind::Other, "An unexpected error occurred."))
    }
}

fn get_today() -> String {
    // 現在のUTC時刻を取得
    let utc_now = Utc::now();

    // 日本時間に変換
    let jst_now = utc_now.with_timezone(&Tokyo);

    // yyyymmdd形式にフォーマット
    let formatted = jst_now.format("%Y%m%d").to_string();

    format!("{}", formatted)
}
