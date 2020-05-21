extern crate chrono;
extern crate csv;
use chrono::{Datelike, Utc};
use regex::Regex;

use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread,
    time::{Duration},

};

pub fn yf_symb_from_url(url: String) -> Option<String> {
    //example 
    let re = Regex::new(r"/chart/(?P<symb>.+).*\?").unwrap();
    if let Some(caps) = re.captures(&url) {
        return Some(caps.name("symb").unwrap().as_str().to_string());
    }
    return None;
}

pub fn symb_from_ndaq_url(url: String) -> Option<String> {
    //example 
    let re = Regex::new(r"/quote/(?P<symb>.+).*/info").unwrap();
    if let Some(caps) = re.captures(&url) {
        return Some(caps.name("symb").unwrap().as_str().to_string());
    }
    return None;
}

//pub fn yf_url(s: Security) -> String {
//    let root = "https://query1.finance.yahoo.com/v8/finance/chart/";
//    // let sfx = "&range=7d&interval=1m";
//    let sfx = "&range=1d&period1={}&period2={}";
//    match s {
//        Security::F(s) => vec![root, &s, "=F?symbol=", &s, sfx].join(""),
//        Security::X(s) => vec![root, &s, "=X?symbol=", &s, sfx].join(""),
//        Security::US(s) => vec![root, &s, "?region=US", sfx].join(""),
//    }
//}

pub fn xueqiu_url(s: String) -> String {
    return format!("https://stock.xueqiu.com/v5/stock/realtime/quotec.json?symbol={}", s.to_string());
}

pub fn writerecs(
    file_name: String,
    header: &[&str],
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
    wtr.write_record(header);
    for r in records.iter() {
        wtr.write_record(r);
    }
    Ok(())
}

pub fn appendrecs(
    file_name: String,
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let file = std::fs::OpenOptions::new().append(true).open(file_name)?;
    let mut wtr = csv::Writer::from_writer(file);
    for r in records.iter() {
        wtr.write_record(r);
    }
    wtr.flush()?;
    Ok(())
}

pub fn appendrec(
    file_name: String,
    rec: csv::StringRecord,
) -> Result<(), Box<dyn Error>> {
    let file = std::fs::OpenOptions::new().append(true).open(file_name)?;
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&rec);
    wtr.flush()?;
    Ok(())
}

pub fn writerecs_strvec(
    file_name: String,
    header: Vec<String>,
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
    wtr.write_record(header);
    for r in records.iter() {
        wtr.write_record(r);
    }
    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn simppath(s: String, sfx: String) -> String {
    //sfx enum x, f, us
    let now = Utc::now();
    return format!(
        "../data/{}_{}_{}_{}_{}.csv",
        s.to_string(),
        sfx.to_string(),
        now.year(),
        now.month(),
        now.day()
    );
}

pub fn chart_headers(s: String) -> Vec<String> {
    let mut headers: Vec<String> = vec!["t".to_string()];

    for elt in YF_HEADER[1..YF_HEADER.len()].iter() {
        headers.push(format!("{}_{}", elt.to_string(), s.to_string()));
    }
    return headers;
}


pub fn yf_symb(url: String) -> Option<Vec<csv::StringRecord>> {
    if let Some(tohlcv) = getters::yf_from_url(url.to_string()) {
        let mut recs: Vec<csv::StringRecord> = tohlcv
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        return Some(recs);
    }
    return None;
}
pub fn sa() -> Result<(), reqwest::Error> {
    let url = "https://seekingalpha.com/get_trending_articles";
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: sa::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = sa::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./sa.csv".to_string(), &SA_HEADER, recs);
    }
    Ok(())
}

pub fn reuters() -> Result<(), csv::Error> {
    let file_name = "./reuters.csv".to_string();
    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
    wtr.write_record(&REUTERS_HEADER);
    for country in REUTERS_COUNTRIES.iter() {
        let url = format!("https://sope.prod.reuters.tv/program/rcom/v1/article-recirc?edition={}&modules=rightrail,ribbon,bottom", country.to_string());
        if let Ok(body) = getters::simple_get(url.to_string()) {
            let root: news::TR = serde_json::from_str(&body.to_string()).unwrap();
            let recs: Vec<csv::StringRecord> = news::TR::to_records(&root)
                .into_iter()
                .map(|x| csv::StringRecord::from(x))
                .collect();
            for r in recs.iter() {
                wtr.write_record(r);
            }
        }
    }
    wtr.flush();
    Ok(())
}

pub fn wsj_videos() {
    let url = "https://video-api.wsj.com/api-video/find_all_videos.asp";
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: news::WSJ = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::WSJ::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./WSJ.csv".to_string(), &WSJ_HEADER, recs);
    }
}

pub fn jpxnews() -> Result<(), reqwest::Error> {
    let url = "https://www.jpx.co.jp/english/news/news_ym_01.json";
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: Vec<jpxnews::Root> = serde_json::from_str(&body.to_string()).unwrap();
        let mut recs: Vec<csv::StringRecord> = Vec::new();
        for r in root.iter() {
            recs.push(csv::StringRecord::from(jpxnews::Root::to_record(r)));
        }
        writerecs("./jpxnews.csv".to_string(), &jpxnews::JPXNewsHeader, recs);
    }
    Ok(())
}

pub fn gsnews() -> Result<(), reqwest::Error> {
    let url = "https://www.goldmansachs.com/insights/insights-articles.json";
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: gs::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = gs::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./gsnews.csv".to_string(), &gs::GS_HEADER, recs);
    }
    Ok(())
}


pub fn nytfeed() -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.nytimes.com/svc/news/v3/content/all/all.json?api-key={}&limit=200",
        keys::NYT_KEY.to_string()
    );
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: news::NYTFeed = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::NYTFeed::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./nytfeed.csv".to_string(), &NYT_FEED_HEADER, recs);
    }
    Ok(())
}


pub fn nytarchive() -> Result<(), csv::Error> {
    let filename = "./nyt_archive.csv".to_string();
    let mut wtr = csv::Writer::from_path(filename)?;
    let nyt_delay: std::time::Duration = Duration::from_millis(6000);

    wtr.write_record(&NYT_ARCHIVE_HEADER);
    for i in 1853..2019 {
        for j in 1..13 {
            let url = format!(
                "https://api.nytimes.com/svc/archive/v1/{}/{}.json?api-key={}",
                i,
                j,
                keys::NYT_KEY.to_string()
            );
            if let Ok(body) = getters::simple_get(url.to_string()) {
                let root: news::NYTArchive = serde_json::from_str(&body.to_string()).unwrap();
                let recs: Vec<csv::StringRecord> = news::NYTArchive::to_records(&root)
                    .into_iter()
                    .map(|x| csv::StringRecord::from(x))
                    .collect();
                for r in recs.iter() {
                    wtr.write_record(r);
                }
                thread::sleep(nyt_delay);
            }
        }
    }
    wtr.flush();
    Ok(())
}

// pub fn steam_listings() -> Result<(), csv::Error> {
//     let url = "https://steamcommunity.com/market/recent?country=US&language=english&currency=1";
//     if let Ok(body) = getters::simple_get(url.to_string()) {
//         let root: steam::Steam = serde_json::from_str(&body.to_string()).unwrap();
//         // println!("{:#?}", root);
//         let recs = steam::Steam::listings(&root)
//             .into_iter()
//             .map(|x| csv::StringRecord::from(x))
//             .collect();
//         let mut heading: Vec<String> = vec!(steam::STEAM_LISTING_HEADER.clone());
//         heading.append(&mut steam::STEAM_ASSET_HEADER.clone().to_vec());
//         writerecs("./steam_new_listings.csv".to_string(), heading, recs);
//     }
//     Ok(())
// }

pub fn steam_purchases() -> Result<(), csv::Error> {
    let url = "https://steamcommunity.com/market/recentcompleted";
    if let Ok(body) = getters::simple_get(url.to_string()) {
        let root: steam::Steam = serde_json::from_str(&body.to_string()).unwrap();
        // println!("{:#?}", root);
        let recs = steam::Steam::purchases(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs(
            "./steam_recent_purchases.csv".to_string(),
            &steam::STEAM_PURCHASE_HEADER2,
            recs,
        );
    }
    Ok(())
}

pub fn lilmatcher(s: Option<String>) -> String {
    match s {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

pub fn lilmatcher_i64(s: Option<i64>) -> String {
    match s {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}

// pub fn steam_sold() -> Result<(), csv::Error> {
//     let url = "https://steamcommunity.com/market/recentcompleted";
//     if let Ok(body) = getters::simple_get(url.to_string()) {
//         let root: steam::Steam = serde_json::from_str(&body.to_string()).unwrap();
//         // let recs = news::WSJ::to_records(&root)
//         //     .into_iter()
//         //     .map(|x| csv::StringRecord::from(x))
//         //     .collect();
//         // writerecs("./steam_sold.csv".to_string(), &STEAM_HEADER, recs);
//     }
// }
// todo , generalizing, refactor
// struct SGen<T>(T);

// pub fn serde_to_recs(body: String, to: SGen<T>) -> SGen<T> {
//     let root: sa::Root = serde_json::from_str(&body.to_string()).unwrap();
//     let recs = sa::Root::to_records(&root)
//         .into_iter()
//         .map(|x| csv::StringRecord::from(x))
//         .collect();
// }

fn regexmain() -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open("rentec_13f.xml")?;
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    let res = vec![
        Regex::new(r"<nameOfIssuer>(?P<val>.+)</nameOfIssuer>.*()").unwrap(),
        Regex::new(r"<titleOfClass>(?P<val>.+)</titleOfClass>.*()").unwrap(),
        Regex::new(r"<cusip>(?P<val>.+)</cusip>.*()").unwrap(),
        Regex::new(r"<value>(?P<val>.+)</value>.*()").unwrap(),
        Regex::new(r"<sshPrnamt>(?P<val>.+)</sshPrnamt>.*()").unwrap(),
        Regex::new(r"<sshPrnamtType>(?P<val>.+)</sshPrnamtType>.*()").unwrap(),
        Regex::new(r"<investmentDiscretion>(?P<val>.+)</investmentDiscretion>.*()").unwrap(),
        Regex::new(r"<otherManager>(?P<val>.+)</otherManager>.*()").unwrap(),
        Regex::new(r"<Sole>(?P<val>.+)</Sole>.*()").unwrap(),
        Regex::new(r"<Shared>(?P<val>.+)</Shared>.*()").unwrap(),
        Regex::new(r"<None>(?P<val>.+)</None>.*()").unwrap(),
    ];
    // buf_reader.read_to_string(&mut contents)?;
    let filenames = read_tickers("./rentec13urls.txt");
    for (i, url) in filenames.iter().enumerate() {
        let mut allcaps: Vec<Vec<String>> = Vec::new();
        let contents = getters::simple_get(url.to_string()).unwrap();
        for re in res.iter() {
            let mut rec: Vec<String> = Vec::new();
            for cap in re.captures_iter(&contents.to_string()) {
                if let Some(val) = cap.name("val") {
                    rec.push(val.as_str().to_string());
                } else {
                    println!("OH FUCK");
                    rec.push("".to_string());
                }
            }
            allcaps.push(rec);
        }
        let path = format!(
            "./ref_data/rentec/regex_rentec_holdings_{}.csv",
            i.to_string()
        );
        let mut wtr = csv::Writer::from_path(path)?;
        let len = allcaps[0].len();
        for vec in allcaps.iter() {
            assert_eq!(len, vec.len());
            let rec = csv::StringRecord::from(vec.clone());
            wtr.write_record(&rec);
        }
    }
    Ok(())
}

