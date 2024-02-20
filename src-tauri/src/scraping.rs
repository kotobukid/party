use std::collections::HashMap;
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
use chrono::Duration;
use serde::{Deserialize, Serialize};

extern crate party;

use party::parse_and_adjust_date;

pub async fn use_cache_or_fetch(root_selector: Vec<String>) -> Result<Vec<WixossPartyDetail>, Box<dyn std::error::Error>> {
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

    let mut event_details: Vec<WixossPartyDetail> = Vec::new();

    let document: Html = Html::parse_document(&body);

    let mut con_map: HashMap<String, String> = HashMap::new();

    root_selector.iter().for_each(|s| {
        let con_name = con_map.get(s);
        let con_name = match con_name {
            Some(con_name) => con_name.clone(),
            None => {
                let con = format!("#option_{}", s);
                let con_selector: Selector = Selector::parse(&con).unwrap();
                let con_name = document.select(&con_selector).next().unwrap().inner_html();
                con_map.insert(s.clone(), con_name.clone());
                con_name
            }
        };

        let selector = format!("#detail_cont_inner_{} .event_box", s).as_str().to_owned();
        let main_selector: Selector = Selector::parse(&selector).unwrap();

        // let content = document.select(&main_selector).next().map(|element| element.inner_html()).unwrap();
        document.select(&main_selector).into_iter().for_each(|c| {
            event_details.push(parse_event(&c.inner_html(), con_name.clone()))
        });
    });
    Ok(event_details)

    // for e in events {
    //     println!("{}\n", e);
    // }
    //
    // Ok(())
}


pub async fn filter_events_in_days_range(root_selector: Vec<String>, days: i64) -> Result<Vec<WixossPartyDetail>, Box<dyn std::error::Error>> {
    let all_events = use_cache_or_fetch(root_selector).await?;

    let now = Utc::now().with_timezone(&Tokyo);
    let future = now + Duration::days(days);

    let filtered_events: Vec<WixossPartyDetail> = all_events.into_iter()
        .filter(|event| {
            event.datetime >= now && event.datetime <= future
        })
        .collect();

    Ok(filtered_events)
}

#[derive(Debug)]
pub struct EventGroup {
    pub date: String,
    pub events: Vec<WixossPartyDetail>,
}

pub async fn group_events_by_day_asc(root_selector: Vec<String>, days: i64) -> Result<Vec<EventGroup>, Box<dyn std::error::Error>> {
    if days < 0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "日数が無効です")));
    }

    let all_events = filter_events_in_days_range(root_selector, days).await?;
    let grouped_events: HashMap<_, Vec<_>> = all_events.into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<_, Vec<_>>, event| {
            acc.entry(event.datetime.date_naive())
                .or_insert_with(|| Vec::new())
                .push(event);
            acc
        });

    let mut result: Vec<EventGroup> = grouped_events.into_iter()
        .map(|(date, events)| EventGroup {
            date: date.format("%Y/%m/%d").to_string(),
            events: events
        })
        .collect();
    result.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(result)
}

fn ensure_cache_dir_exists(cache_dir: &str) -> std::io::Result<()> {
    fs::create_dir_all(cache_dir)
}

pub(crate) async fn save_to_cache(cache_dir: &str, cache_filename: &str, content: &str) -> std::io::Result<()> {
    ensure_cache_dir_exists(cache_dir).unwrap();
    let path = Path::new(cache_dir).join(cache_filename);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WixossPartyDetail {
    pub name: String,
    pub con: String,
    pub shop_name: String,
    pub shop_link: String,
    pub time_s: String,
    pub datetime: DateTime<Local>,
    pub format: String,
}

impl Display for WixossPartyDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\t{}\n\t{}\n\t{}\n\t{}", self.name, self.shop_name, self.shop_link, self.time_s, self.format)
        // write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

fn parse_event<'a>(src: &'a str, con_name: String) -> WixossPartyDetail {
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
    let time: String = extract_text(&selector_time);

    WixossPartyDetail {
        name: extract_text(&selector_event_name),
        con: con_name,
        shop_name: extract_text(&selector_shop_name),
        shop_link: html.select(&selector_shop_name).next().unwrap().value().attr("href").unwrap_or("").to_string(),
        time_s: time.clone(),
        datetime: parse_and_adjust_date(&time.trim_end_matches('〜')),
        format: extract_text(&selector_format),
    }
}

pub(crate) fn cache_check(dir: &str, filename: &str) -> Result<String, std::io::Error> {
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

pub(crate) fn get_today() -> String {
    // 現在のUTC時刻を取得
    let utc_now = Utc::now();

    // 日本時間に変換
    let jst_now = utc_now.with_timezone(&Tokyo);

    // yyyymmdd形式にフォーマット
    let formatted = jst_now.format("%Y%m%d").to_string();

    format!("{}", formatted)
}