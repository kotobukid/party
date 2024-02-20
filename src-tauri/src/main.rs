// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scraping;
mod shiromado;

use dotenv::dotenv;
use std::env;
use std::fmt::{Display, Formatter};
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
    // category: shiromado::ShiromadoCategory, // => format
}
impl Display for GenericEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl From<WixossPartyDetail> for GenericEvent {
    fn from(value: WixossPartyDetail) -> Self {
        GenericEvent {
            name: value.name,
            time_s: value.time_s,
            state: value.con,
            format: value.format,
            url: value.shop_link,
            shop: value.shop_name,
            date: value.datetime.to_string()
        }
    }
}

impl From<ShiromadoEvent> for GenericEvent {
    fn from(value: ShiromadoEvent) -> Self {
        GenericEvent {
            name: value.name,
            time_s: value.time_s.clone(),
            state: value.state.to_string(),
            format: value.category.to_string(),
            url: value.url,
            shop: value.community,
            date: value.date
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
