use chrono::{Date, NaiveDate, Utc};
use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};

use std::collections::HashMap;

use finox::{news::wsj, roses};

const F: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'=')
    .remove(b'&')
    .remove(b'_')
    .remove(b'-')
    .remove(b'?');

#[tokio::main]
async fn main() {
    // fix this to read in the csvs. date isn't actually needed though
    // https://www.wsj.com/news/archive/2003/12/22?id=SB852113200493047500&type=article%7Cdnsa
    let root = "https://www.wsj.com/news/archive/";
    //let test_ids = vec![
    //    "SB852112225564165000",
    //    "SB12529005974621074610804586506800603874650",
    //];

    let test_urls = roses::read_tickers("../data/news/wsj/article_ids_index.txt")
        .iter()
        .rev()
        .map(|x| format!("{}?id={}&type=article%7Cdnsa", root, x))
        .collect::<Vec<_>>();
    let data = finox::fetch_into::<wsj::WSJArticleRoot>(test_urls[0..100].to_vec())
        .await
        .into_iter()
        .flatten() //
        .map(|x| finox::HasRec::to_rec(&x))
        .collect::<Vec<Vec<String>>>(); //<Vec<wsj::WSJArticleRoot>>();

    println!("# {:#?}", data);
}

async fn get_ids() {
    let start = NaiveDate::from_ymd(1997, 1, 1);
    let today = Date::naive_utc(&Utc::today());
    let days = days_between(start, today);
    let urls = days.iter().map(|x| full_fmt(x)).collect::<Vec<_>>();

    let mut hm: HashMap<String, String> = HashMap::new();

    for d in days.iter() {
        hm.insert(d.format("%Y_%m_%d").to_string(), full_fmt(d));
    }

    let data = finox::fetch_write::<wsj::WSJArchive>(
        hm,
        "../data/news/wsj/archive_ids/",
        &finox::headers::WSJ_ARCHIVE_HEADER,
    )
    .await
    .unwrap();

    println!("# {:#?}", data.len());
}

fn test_full_link_fmt() {
    let d = Date::naive_utc(&Utc::today());
    let full = full_fmt(&d);
    println!("{}", full);
}

fn test_days_between() {
    let r = Some("https://www.wsj.com/news/archive/");
    let start = NaiveDate::from_ymd(1997, 1, 1);
    let today = Date::naive_utc(&Utc::today());
    let days = days_between(start, today)
        .iter()
        .map(|x| to_wsj_fmt(r, *x))
        .collect::<Vec<_>>();
    println!(
        "# days inclusive between {:#?} and {:#?} is {:#?}!",
        start,
        today,
        days.len()
    );
}

// given (t1, t2) st t1 <= t2, returns inclusive days between the interval
fn days_between(mut t1: NaiveDate, t2: NaiveDate) -> Vec<NaiveDate> {
    let mut days = vec![];
    while t1 <= t2 {
        days.push(t1);
        t1 = t1.succ();
    }
    days
}

// adding the root optional is trashy
fn to_wsj_fmt(r: Option<&str>, d: NaiveDate) -> String {
    match r {
        Some(root) => format!("{}{}", root, d.format("%Y/%m/%d")).to_string(),
        _ => d.format("%Y/%m/%d").to_string(),
    }
}

// for the IDs
fn full_fmt(d: &NaiveDate) -> String {
    let r = "https://www.wsj.com/news/archive/";
    let str_d = to_wsj_fmt(None, *d);
    let mut q = format!(
        r#"?id={{"params": {{ "timeout": "2000", "query":"","count": "200","max-date": "{}","min-date": "{}"}},"clientId": "grandcanyon","database": "wsjie"}}&type=dnsasearch_full"#,
        str_d, str_d
    );
    q = percent_encode(q.as_bytes(), F).to_string();
    format!("{}{}{}", r, d, q)
}
