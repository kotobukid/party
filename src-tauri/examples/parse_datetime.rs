extern crate chrono;

use chrono::Timelike;
use chrono::{Local, NaiveDateTime, TimeZone, Datelike};

fn parse_and_adjust_date(input: &str) -> chrono::DateTime<Local> {  // 月日の文字列を、未来方向に限定することで年を補完する
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

fn print_parsed(input: &str) -> () {
    // 末尾の〜を取り除く
    let clean_input = input.trim_end_matches('〜');
    let datetime = parse_and_adjust_date(clean_input);
    println!("Parsed and adjusted datetime: {}", datetime);
    ()
}

fn main() {
    print_parsed("02月29日 17時00分〜");
    print_parsed("01月29日 12時00分〜");
    print_parsed("02月08日 00時30分〜");
    print_parsed("02月08日00時30分〜");  // スペースがない
    print_parsed("02月08日 00時30分");  //  〜が無い
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_adjust_date() {
        let now = Local::now();
        let current_year = now.year();
        let current_month = now.month();

        // ケース1: 現在の月よりも後の日付（年の調整が不要）
        let case1 = "02月29日 17時00分〜";
        let expected_year_case1 = if current_month > 2 { current_year + 1 } else { current_year };
        assert_eq!(parse_and_adjust_date(case1.trim_end_matches('〜')).year(), expected_year_case1);

        // ケース2: 現在の月よりも前の日付（年を1つ上げる）
        let case2 = "01月29日 12時00分〜";
        let expected_year_case2 = if current_month > 1 { current_year + 1 } else { current_year };
        assert_eq!(parse_and_adjust_date(case2.trim_end_matches('〜')).year(), expected_year_case2);

        // ケース3: 現在の月と同じだが、日付が後（年の調整が不要）
        let case3 = format!("{:02}月08日 00時30分〜", current_month);
        assert_eq!(parse_and_adjust_date(case3.trim_end_matches('〜')).year(), current_year);

        // ケース4: 「〜」が含まれない場合
        let case4 = "02月08日 00時30分";
        let expected_year_case4 = if current_month > 2 { current_year + 1 } else { current_year };
        assert_eq!(parse_and_adjust_date(case4).year(), expected_year_case4);
    }
}
