use chrono::{Datelike, NaiveDate, NaiveDateTime};

fn interpret_date(date_str: &str, now: NaiveDateTime) -> Option<String> {
    if date_str.len() != 4 {
        return None;
    }

    let mut year = now.year();
    let month = now.month();

    let target_month: u32 = date_str[0..2].parse().ok()?;
    let target_day: u32 = date_str[2..4].parse().ok()?;

    if target_month != month {
        year += 1;
    }

    let date = NaiveDate::from_ymd_opt(year, target_month, target_day);
    match date {
        Some(d) => Some(format!("{}", d.format("%Y%m%d"))),
        None => None
    }
}

fn parse_date_or_unknown(date_str: &str, now: NaiveDateTime) -> String {
    let result = interpret_date(date_str, now);
    result.unwrap_or_else(|| "Unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::{interpret_date, parse_date_or_unknown};
    use chrono::{NaiveDate};

    #[test]
    fn test_interpret_date() {
        assert_eq!(interpret_date("0120", NaiveDate::from_ymd_opt(2023, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), Some("20230120".to_string()));
        assert_eq!(interpret_date("0120", NaiveDate::from_ymd_opt(2023, 12, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), Some("20240120".to_string()));
        assert_eq!(interpret_date("0120", NaiveDate::from_ymd_opt(2023, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), Some("20230120".to_string()));
        assert_eq!(parse_date_or_unknown("0229", NaiveDate::from_ymd_opt(2023, 2, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "Unknown".to_string());
        assert_eq!(parse_date_or_unknown("0229", NaiveDate::from_ymd_opt(2024, 2, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "20240229".to_string());
        assert_eq!(parse_date_or_unknown("1301", NaiveDate::from_ymd_opt(2023, 12, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "Unknown".to_string());
        assert_eq!(parse_date_or_unknown("1232", NaiveDate::from_ymd_opt(2023, 12, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "Unknown".to_string());
        assert_eq!(parse_date_or_unknown("abcd", NaiveDate::from_ymd_opt(2023, 12, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "Unknown".to_string());
        assert_eq!(parse_date_or_unknown("120", NaiveDate::from_ymd_opt(2023, 12, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "Unknown".to_string());
        assert_eq!(parse_date_or_unknown("", NaiveDate::from_ymd_opt(2023, 12, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()), "Unknown".to_string());
    }
}

fn main() {
    let a = parse_date_or_unknown("0228", NaiveDate::from_ymd_opt(2023, 2, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());
    println!("{}", a)
}