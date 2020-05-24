extern crate chrono;
extern crate csv;
use chrono::Utc;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread,
    time::Duration,
};

pub const DELAY: std::time::Duration = Duration::from_millis(10);

pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36";

#[tokio::main]
pub async fn simple_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.to_string())
        .build()?;
    let res = client.get(&url).send().await?;
    thread::sleep(DELAY);
    let body = res.text().await?;
    // println!("{}: {:#?}", url, body);
    println!("{}", url);
    Ok(body)
}

#[tokio::main]
pub async fn simple_json(url: String) -> Result<::serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.to_string())
        .build()?;

    client
        .get(&url)
        .send()
        .await?
        .json::<::serde_json::Value>() // CHANGE TYPE
        .await
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
        "../data/{}_{}_{}.csv",
        s.to_string(),
        sfx.to_string(),
        Utc::now().to_rfc3339(),
    );
}

pub fn chart_headers(s: String) -> Vec<String> {
    let mut headers: Vec<String> = vec!["t".to_string()];

    for elt in crate::YF_HEADER[1..crate::YF_HEADER.len()].iter() {
        headers.push(format!("{}_{}", elt.to_string(), s.to_string()));
    }
    return headers;
}

pub fn sa() -> Result<(), reqwest::Error> {
    let url = "https://seekingalpha.com/get_trending_articles";
    if let Ok(body) = simple_get(url.to_string()) {
        let root: crate::sa::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = crate::sa::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./sa.csv".to_string(), &crate::SA_HEADER, recs);
    }
    Ok(())
}

pub fn reuters() -> Result<(), csv::Error> {
    let file_name = "./reuters.csv".to_string();
    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
    wtr.write_record(&crate::REUTERS_HEADER);
    for country in crate::REUTERS_COUNTRIES.iter() {
        let url = format!("https://sope.prod.reuters.tv/program/rcom/v1/article-recirc?edition={}&modules=rightrail,ribbon,bottom", country.to_string());
        if let Ok(body) = simple_get(url.to_string()) {
            let root: crate::news::TR = serde_json::from_str(&body.to_string()).unwrap();
            let recs: Vec<csv::StringRecord> = crate::news::TR::to_records(&root)
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
    if let Ok(body) = simple_get(url.to_string()) {
        let root: crate::news::WSJ = serde_json::from_str(&body.to_string()).unwrap();
        let recs = crate::news::WSJ::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./WSJ.csv".to_string(), &crate::WSJ_HEADER, recs);
    }
}

//pub fn jpxnews() -> Result<(), reqwest::Error> {
//    let url = "https://www.jpx.co.jp/english/news/news_ym_01.json";
//    if let Ok(body) = simple_get(url.to_string()) {
//        let root: Vec<crate::jpxnews::Root> = serde_json::from_str(&body.to_string()).unwrap();
//        let mut recs: Vec<csv::StringRecord> = Vec::new();
//        for r in root.iter() {
//            recs.push(csv::StringRecord::from(crate::jpxnews::Root::to_record(r)));
//        }
//        writerecs("./jpxnews.csv".to_string(), &jpxnews::JPXNewsHeader, recs);
//    }
//    Ok(())
//}

pub fn gsnews() -> Result<(), reqwest::Error> {
    let url = "https://www.goldmansachs.com/insights/insights-articles.json";
    if let Ok(body) = simple_get(url.to_string()) {
        let root: crate::gs::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = crate::gs::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./gsnews.csv".to_string(), &crate::gs::GS_HEADER, recs);
    }
    Ok(())
}

pub fn nytfeed() -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.nytimes.com/svc/news/v3/content/all/all.json?api-key={}&limit=200",
        crate::keys::NYT_KEY.to_string()
    );
    if let Ok(body) = simple_get(url.to_string()) {
        let root: crate::news::NYTFeed = serde_json::from_str(&body.to_string()).unwrap();
        let recs = crate::news::NYTFeed::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        writerecs("./nytfeed.csv".to_string(), &crate::NYT_FEED_HEADER, recs);
    }
    Ok(())
}

pub fn nytarchive() -> Result<(), csv::Error> {
    let filename = "./nyt_archive.csv".to_string();
    let mut wtr = csv::Writer::from_path(filename)?;
    let nyt_delay: std::time::Duration = Duration::from_millis(6000);

    wtr.write_record(&crate::NYT_ARCHIVE_HEADER);
    for i in 1853..2019 {
        for j in 1..13 {
            let url = format!(
                "https://api.nytimes.com/svc/archive/v1/{}/{}.json?api-key={}",
                i,
                j,
                crate::keys::NYT_KEY.to_string()
            );
            if let Ok(body) = simple_get(url.to_string()) {
                let root: crate::news::NYTArchive =
                    serde_json::from_str(&body.to_string()).unwrap();
                let recs: Vec<csv::StringRecord> = crate::news::NYTArchive::to_records(&root)
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

//pub fn steam_purchases() -> Result<(), csv::Error> {
//    let url = "https://steamcommunity.com/market/recentcompleted";
//    if let Ok(body) = getters::simple_get(url.to_string()) {
//        let root: steam::Steam = serde_json::from_str(&body.to_string()).unwrap();
//        // println!("{:#?}", root);
//        let recs = steam::Steam::purchases(&root)
//            .into_iter()
//            .map(|x| csv::StringRecord::from(x))
//            .collect();
//        writerecs(
//            "./steam_recent_purchases.csv".to_string(),
//            &steam::STEAM_PURCHASE_HEADER2,
//            recs,
//        );
//    }
//    Ok(())
//}
//
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
    let filenames = read_tickers("../ref_data/rentec13urls.txt");
    for (i, url) in filenames.iter().enumerate() {
        let mut allcaps: Vec<Vec<String>> = Vec::new();
        let contents = simple_get(url.to_string()).unwrap();
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


