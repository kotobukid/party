use std::fmt::{Display, Formatter};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum State {
    Hokkaido,
    Aomori,
    Miyagi,
    Iwate,
    Akita,
    Yamagata,
    Fukushima,
    Tokyo,
    Kanagawa,
    Saitama,
    Chiba,
    Ibaragi,
    Tochigi,
    Gunma,
    Yamanashi,
    Nagano,
    Nigata,
    Toyama,
    Ishikawa,
    Fukui,
    Gifu,
    Shizuoka,
    Aichi,
    Mie,
    Shiga,
    Kyoto,
    Osaka,
    Hyogo,
    Nara,
    Wakayama,
    Tottori,
    Shimane,
    Okayama,
    Hiroshima,
    Yamaguchi,
    Tokushima,
    Kagawa,
    Ehime,
    Kochi,
    Fukuoka,
    Saga,
    Nagasaki,
    Kumamoto,
    Oita,
    Miyazaki,
    Kagoshima,
    Okinawa,
    Online,
    Other(String),
}

impl From<String> for State {
    fn from(source: String) -> Self {
        let string: &str = &source.trim();

        match string {
            "北海道" => State::Hokkaido,
            "青森県" => State::Aomori,
            "宮城県" => State::Miyagi,
            "岩手県" => State::Iwate,
            "秋田県" => State::Akita,
            "山形県" => State::Yamagata,
            "福島県" => State::Fukushima,
            "東京都" => State::Tokyo,
            "神奈川県" => State::Kanagawa,
            "埼玉県" => State::Saitama,
            "千葉県" => State::Chiba,
            "茨城県" => State::Ibaragi,
            "栃木県" => State::Tochigi,
            "群馬県" => State::Gunma,
            "山梨県" => State::Yamanashi,
            "長野県" => State::Nagano,
            "新潟県" => State::Nigata,
            "富山県" => State::Toyama,
            "石川県" => State::Ishikawa,
            "福井県" => State::Fukui,
            "岐阜県" => State::Gifu,
            "静岡県" => State::Shizuoka,
            "愛知県" => State::Aichi,
            "三重県" => State::Mie,
            "滋賀県" => State::Shiga,
            "京都府" => State::Kyoto,
            "大阪府" => State::Osaka,
            "兵庫県" => State::Hyogo,
            "奈良県" => State::Nara,
            "和歌山県" => State::Wakayama,
            "鳥取県" => State::Tottori,
            "島根県" => State::Shimane,
            "岡山県" => State::Okayama,
            "広島県" => State::Hiroshima,
            "山口県" => State::Yamaguchi,
            "徳島県" => State::Tokushima,
            "香川県" => State::Kagawa,
            "愛媛県" => State::Ehime,
            "高知県" => State::Kochi,
            "福岡県" => State::Fukuoka,
            "佐賀県" => State::Saga,
            "長崎県" => State::Nagasaki,
            "熊本県" => State::Kumamoto,
            "大分県" => State::Oita,
            "宮崎県" => State::Miyazaki,
            "鹿児島県" => State::Kagoshima,
            "沖縄県" => State::Okinawa,
            "オンライン" => State::Online,

            "hokkaido" => State::Hokkaido,
            "aomori" => State::Aomori,
            "miyagi" => State::Miyagi,
            "iwate" => State::Iwate,
            "akita" => State::Akita,
            "yamagata" => State::Yamagata,
            "fukushima" => State::Fukushima,
            "tokyo" => State::Tokyo,
            "kanagawa" => State::Kanagawa,
            "saitama" => State::Saitama,
            "chiba" => State::Chiba,
            "ibaragi" => State::Ibaragi,
            "tochigi" => State::Tochigi,
            "gunma" => State::Gunma,
            "yamanashi" => State::Yamanashi,
            "nagano" => State::Nagano,
            "nigata" => State::Nigata,
            "toyama" => State::Toyama,
            "ishikawa" => State::Ishikawa,
            "fukui" => State::Fukui,
            "gifu" => State::Gifu,
            "shizuoka" => State::Shizuoka,
            "aichi" => State::Aichi,
            "mie" => State::Mie,
            "shiga" => State::Shiga,
            "kyoto" => State::Kyoto,
            "osaka" => State::Osaka,
            "hyogo" => State::Hyogo,
            "nara" => State::Nara,
            "wakayama" => State::Wakayama,
            "tottori" => State::Tottori,
            "shimane" => State::Shimane,
            "okayama" => State::Okayama,
            "hiroshima" => State::Hiroshima,
            "yamaguchi" => State::Yamaguchi,
            "tokushima" => State::Tokushima,
            "kagawa" => State::Kagawa,
            "ehime" => State::Ehime,
            "kochi" => State::Kochi,
            "fukuoka" => State::Fukuoka,
            "saga" => State::Saga,
            "nagasaki" => State::Nagasaki,
            "kumamoto" => State::Kumamoto,
            "oita" => State::Oita,
            "miyazaki" => State::Miyazaki,
            "kagoshima" => State::Kagoshima,
            "okinawa" => State::Okinawa,
            _ => State::Other(source)
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Hokkaido => "北海道",
            Self::Aomori => "青森県",
            Self::Miyagi => "宮城県",
            Self::Iwate => "岩手県",
            Self::Akita => "秋田県",
            Self::Yamagata => "山形県",
            Self::Fukushima => "福島県",
            Self::Tokyo => "東京都",
            Self::Kanagawa => "神奈川県",
            Self::Saitama => "埼玉県",
            Self::Chiba => "千葉県",
            Self::Ibaragi => "茨城県",
            Self::Tochigi => "栃木県",
            Self::Gunma => "群馬県",
            Self::Yamanashi => "山梨県",
            Self::Nagano => "長野県",
            Self::Nigata => "新潟県",
            Self::Toyama => "富山県",
            Self::Ishikawa => "石川県",
            Self::Fukui => "福井県",
            Self::Gifu => "岐阜県",
            Self::Shizuoka => "静岡県",
            Self::Aichi => "愛知県",
            Self::Mie => "三重県",
            Self::Shiga => "滋賀県",
            Self::Kyoto => "京都府",
            Self::Osaka => "大阪府",
            Self::Hyogo => "兵庫県",
            Self::Nara => "奈良県",
            Self::Wakayama => "和歌山県",
            Self::Tottori => "鳥取県",
            Self::Shimane => "島根県",
            Self::Okayama => "岡山県",
            Self::Hiroshima => "広島県",
            Self::Yamaguchi => "山口県",
            Self::Tokushima => "徳島県",
            Self::Kagawa => "香川県",
            Self::Ehime => "愛媛県",
            Self::Kochi => "高知県",
            Self::Fukuoka => "福岡県",
            Self::Saga => "佐賀県",
            Self::Nagasaki => "長崎県",
            Self::Kumamoto => "熊本県",
            Self::Oita => "大分県",
            Self::Miyazaki => "宮崎県",
            Self::Kagoshima => "鹿児島県",
            Self::Okinawa => "沖縄県",
            Self::Online => "オンライン",
            Self::Other(ss) => ss
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_state_hokkaido() {
        assert_eq!(State::from("北海道 ".into()), State::Hokkaido);
    }

    #[test]
    fn test_match_state_online() {
        assert_eq!(State::from("オンライン".into()), State::Online);
    }

    #[test]
    fn test_match_state_non_prefecture() {
        assert_eq!(State::from("モンサンミシェル".into()), State::Other("モンサンミシェル".to_string()));
    }
}


pub enum ShiromadoCategory {
    Gachi,
    Casual,
    Unknown,
}

impl ShiromadoCategory {
    pub fn new(cat: &str) -> Self {
        match cat {
            "楽しくばとる" => Self::Casual,
            "ガチばとる" => Self::Gachi,
            _ => Self::Unknown
        }
    }
}

impl Display for ShiromadoCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   ShiromadoCategory::Gachi => "ガチばとる".to_string(),
                   ShiromadoCategory::Casual => "楽しくばとる".to_string(),
                   ShiromadoCategory::Unknown => "(不明)".to_string()
               }
        )
    }
}

pub struct ShiromadoEvent {
    pub name: String,
    pub url: String,
    pub area: String,
    pub state: State,
    pub community: String,
    pub location: String,
    pub time_s: String,
    pub date: String,
    pub owner: String,
    pub players: String,
    pub category: ShiromadoCategory,
}

impl Display for ShiromadoEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n {}\n コミュニティ: {}\n URL: {}\n 開催日: {} {}\n 主催者名: {}\n 開催場所: {}\n 定員: {}\n プラン: {}\n",
               self.state,
               self.name,
               self.community,
               self.url,
               self.date,
               self.time_s,
               self.owner,
               self.location,
               self.players,
               self.category)
    }
}

pub fn shiromado() -> std::io::Result<()> {
    let mut file = File::open("./cache/sample.html")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let events = parse_html(&content);

    // 日付でグルーピング
    let mut grouped_events: HashMap<String, Vec<&ShiromadoEvent>> = HashMap::new();
    for event in &events {
        grouped_events.entry(event.date.clone()).or_insert_with(Vec::new).push(event);
    }

    // 日付で昇順にソートして出力
    let mut dates: Vec<&String> = grouped_events.keys().collect();
    dates.sort(); // 日付を昇順にソート
    for date in dates {
        println!("# Date: {}", date);
        for event in &grouped_events[date] {
            println!("{}", event);
        }
    }

    Ok(())
}

macro_rules! text_to_string {
    ($node:expr) => {
        $node.text().collect::<Vec<_>>().join("").trim().to_string().replace("\n", "")
    };
}

fn parse_html(body: &str) -> Vec<ShiromadoEvent> {
    let document = Html::parse_document(&body);
    let select = Selector::parse("div.detail_cont_inner *").unwrap();

    let mut current_area: String = String::from("Unknown");

    let title_selector = Selector::parse("dt.title span").unwrap();
    let community_selector = Selector::parse("dt.title a").unwrap();
    let event_txt_selector = Selector::parse(".event_txt dd, .event_txt_hidden dd").unwrap();

    let mut events: Vec<ShiromadoEvent> = Vec::new();

    for element in document.select(&select) {
        let tag_name = element.value().name();
        match tag_name {
            "h5" => {
                let text: String = element.text().collect();
                current_area = text.clone();
            }
            "dl" => {
                if let Some(class_attr) = element.value().attr("class") {
                    if class_attr == "com_event" {
                        let title = &element.select(&title_selector).next().unwrap();
                        let community = &element.select(&community_selector).next().unwrap();
                        let url: String = community.attr("href").unwrap_or("").to_string();

                        let mut texts = element.select(&event_txt_selector);
                        let time_s = texts.next().unwrap();
                        let location = texts.next().unwrap();
                        let owner = texts.next().unwrap();
                        let players = texts.next().unwrap();
                        let category = texts.next().unwrap();
                        let time = text_to_string!(time_s);

                        let ev: ShiromadoEvent = ShiromadoEvent {
                            name: text_to_string!(title),
                            area: current_area.trim().to_string(),
                            state: State::from(current_area.trim().to_string()),
                            community: text_to_string!(community),
                            url,
                            time_s: time.clone(),
                            date: to_sortable_date(&time).unwrap_or("Unknown".to_string()),
                            location: text_to_string!(location),
                            owner: text_to_string!(owner),
                            players: text_to_string!(players),
                            category: ShiromadoCategory::new(&text_to_string!(category)),
                        };

                        // println!("{}", ev);
                        events.push(ev);
                    } else {
                        // println!("dl without com_event");
                    }
                } else {
                    // println!("dl without class");
                }
            }
            _ => (),
        };
    }

    events
}

pub fn to_sortable_date(target: &str) -> Result<String, &'static str> {
    let re = regex::Regex::new(r"(\d+)月\s*(\d+)日").map_err(|_| "Unknown")?;
    let mat = re.captures(target).ok_or("Unknown")?;

    let month = mat.get(1)
        .ok_or("Unknown")?
        .as_str()
        .trim()
        .parse::<u32>()
        .map_err(|_| "Unknown")?;
    let month = format!("{:02}", month);

    let day = mat.get(2)
        .ok_or("Unknown")?
        .as_str()
        .trim()
        .parse::<u32>()
        .map_err(|_| "Unknown")?;
    let day = format!("{:02}", day);

    let res = format!("{}{}", month, day);
    Ok(res)
}

pub async fn use_cache_or_fetch(root_selector: Vec<String>) -> Result<Vec<ShiromadoEvent>, Box<dyn std::error::Error>> {
    let cache_dir = "./cache";
    let cache_filename = format!("{}_s.txt", crate::scraping::get_today());

    let body: String = match crate::scraping::cache_check(&cache_dir, &cache_filename) {
        Ok(content) => content,
        _ => {
            let url = "https://www.takaratomy.co.jp/products/wixoss/campaign/cp_210715/";

            let client: Client = Client::new();
            let res = client.get(url)
                .header(reqwest::header::COOKIE, "wixAge=conf")
                .send().await?;
            let body = res.text().await?;

            if crate::scraping::save_to_cache(&cache_dir, &cache_filename, &body).await.is_ok() {
                // println!("キャッシュに保存成功");
            } else {
                println!("キャッシュに保存失敗");
            }

            body
        }
    };

    let all_events: Vec<ShiromadoEvent> = parse_html(&body);
    let categories: HashSet<String> = root_selector.into_iter().map(|c| {
        let state = State::from(c);
        match state {
            State::Other(o) => {
                match o.as_str() {
                    "online" => "オンライン".to_string(),
                    _ => o
                }
            },
            _ => state.to_string()
        }
    }).collect();

    let filtered_events: Vec<ShiromadoEvent> = all_events.into_iter()
        .filter(|event| categories.contains(&event.state.to_string()))
        .collect();

    Ok(filtered_events)
}