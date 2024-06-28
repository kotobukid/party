// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scraping;
mod shiromado;

use dotenv::dotenv;
use std::env;
use std::fmt::{Display, Formatter};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use serde::Serialize;
use crate::scraping::WixossPartyDetail;
use crate::shiromado::{ShiromadoEvent};

enum WXEvent {
    Party(WixossPartyDetail),
    Shiromado(ShiromadoEvent)
}

#[derive(Serialize)]
struct GenericEvent {
    // shared
    name: String,
    time_s: String,

    // new
    shop: String,
    community: String,

    // wp
    // con: String, => state
    // shop_name: String, => shop
    // shop_link: String, => url
    // datetime: DateTime<Local>, => date(Date)
    format: String, // => format

    // shiromado
    url: String,    // <= shop_link
//-    area: String,
    state: String,   // <_ con
    // community: String,  => shop
//-    location: String,
    date: String, // <= datetime
    //- owner: String,
    //- players: String,
    category: String, // => format
}
impl Display for GenericEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<WixossPartyDetail> for GenericEvent {
    fn from(value: WixossPartyDetail) -> Self {
        let date =  value.datetime.format("%Y-%m-%d").to_string();

        GenericEvent {
            name: value.name,
            time_s: value.time_s,
            state: value.con,
            format: value.format,
            category: "".to_string(),
            url: value.shop_link,
            community: "".to_string(),
            shop: value.shop_name,
            date
        }
    }
}

fn interpret_date(date_str: &str, now: NaiveDateTime) -> Option<String> {
    if date_str.len() != 4 {
        return None;
    }

    let mut year = now.year();
    let month = now.month();

    let target_month: u32 = date_str[0..2].parse().ok()?;
    let target_day: u32 = date_str[2..4].parse().ok()?;

    if target_month < month {
        year += 1;
    }

    let date = NaiveDate::from_ymd_opt(year, target_month, target_day);

    match date {
        Some(d) => Some(format!("{}", d.format("%Y-%m-%d"))),
        None => None
    }
}

fn parse_date_or_unknown(date_str: &str, now: NaiveDateTime) -> String {
    let result = interpret_date(date_str, now);
    result.unwrap_or_else(|| "Unknown".to_string())
}


impl From<ShiromadoEvent> for GenericEvent {
    fn from(value: ShiromadoEvent) -> Self {
        let today = Local::now();
        let start_of_day = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let date = parse_date_or_unknown(&value.date, start_of_day);

        GenericEvent {
            name: value.name,
            time_s: value.time_s.clone(),
            state: value.state.to_string(),
            format: value.format.to_string(),
            category: value.category.to_string(),
            url: value.url,
            shop: value.community,
            community: value.location,
            date
        }
    }
}
#[tauri::command]
async fn fetch_events() -> Result<Vec<GenericEvent>, tauri::InvokeError> {
    dotenv().ok();

    let key = "WX_SEL";
    let root_selectors: Vec<String> = dotenv::var(key).unwrap_or("fukuoka".to_string()).split(',').into_iter().map(|s| s.trim().to_owned()).collect();

    let mut events: Vec<GenericEvent> = Vec::new();

    let shiromado_events = shiromado::use_cache_or_fetch(root_selectors.clone()).await.map_err(|e| tauri::InvokeError::from(e.to_string()))?;
    events.extend(shiromado_events.into_iter().map(GenericEvent::from));

    let scraping_events = scraping::use_cache_or_fetch(root_selectors).await.map_err(|e| tauri::InvokeError::from(e.to_string()))?;
    events.extend(scraping_events.into_iter().map(GenericEvent::from));
    
    // events.iter().for_each(|e| {
    //     println!("{}", e)
    // });
    
    Ok(events)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_events])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
