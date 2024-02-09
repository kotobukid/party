use chrono::{Local, NaiveDateTime, TimeZone};
use chrono::{Timelike, Datelike};

pub fn parse_and_adjust_date(input: &str) -> chrono::DateTime<Local> {  // 月日の文字列を、未来方向に限定することで年を補完する
    // 現在の年月を取得
    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month();

    // 入力文字列をパースするための形式を指定
    let format = "%m月%d日 %H時%M分";
    // パースしてNaiveDateTimeオブジェクトを生成
    let naive_datetime = NaiveDateTime::parse_from_str(&format!("{} {}", current_year, input), &format!("{} {}", "%Y", format))
        .expect("Failed to parse date");

    // パースした月が現在の月より小さい場合、年を1つ上げる
    let adjusted_year = if naive_datetime.month() < current_month {
        current_year + 1
    } else {
        current_year
    };

    // 補正した年でDateTimeオブジェクトを生成
    Local.with_ymd_and_hms(adjusted_year, naive_datetime.month(), naive_datetime.day(), naive_datetime.hour(), naive_datetime.minute(), 0).unwrap()
}