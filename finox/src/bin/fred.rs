#[allow(dead_code)]
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate url;

use futures::stream::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;
use std::{thread, time, fs::{File}, path::Path, io::{prelude::*, BufReader}};
use url::Url;

//use std::sync::{Arc, Mutex};

pub const WRITE_PATH: &str = "../data/fred/";
pub const DELAY: time::Duration = time::Duration::from_millis(500);

fn extract_id(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"id=(?P<id>\w*)&").unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("id").map(|login| login.as_str()))

}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let path = "fred/category/series";
    //let path = "fred/series/observations";
    //let q = "series_id=";
    let q = "category_id=";
    //println!("{:?}", url.path());
    //let ids = read_tickers("../data/fred/fred_series_ids.txt");
    //let urls = gen_queries(ids, path, q);
    let urls = gen_queries(path, q);
    println!("{:#?}", urls);
    let roots = lil_fetch(urls.iter().map(|x| x.to_string()).collect::<Vec<_>>()).await;
    println!("{:#?}", roots);

    Ok(())
}

pub async fn lil_fetch(urls: Vec<String>) -> Vec<String> {
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            if let Ok(root) = res.json::<CategoryRoot>().await {
                println!("url: {}, {:#?}", url.clone(), root);
                thread::sleep(DELAY);
                let recs = root.to_recs();
                let id = extract_id(&url).unwrap();
                let fp = format!("{}category/{}_category.csv", WRITE_PATH, id);
                if let Ok(_) = write_csv(
                    &fp,
                    recs,
                    //OBS_HEADER
                    SERIES_HEADER
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>(),
                ){
                    return Some(id.to_string());
                }
            return None;

            } else {
                println!("serialize err {}", url.clone());
                return None;
            }
        }
        println!("response err: {}", url.clone());
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<String>>>()
    .await;
    return fetches.into_iter().flatten().collect::<Vec<String>>();
}

pub fn write_csv(fp: &str, recs: Vec<Vec<String>>, header: Vec<String>) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(fp)?;
    let len = header.len();
    wtr.write_record(header)?;
    wtr.flush()?;
    for row in recs.iter() {
        assert_eq!(len, row.len()); // perf hit?
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

fn gen_queries(
    //ids: Vec<String>, 
    path: &str, q: &str) -> Vec<Url> {
    let mut urls: Vec<Url> = vec![];
    let root = Url::parse("https://api.stlouisfed.org");
    //for i in ids.iter() {
    for i in 30000..40000 {
        if let Ok(r) = root.clone() {
            let q = format!(
                "{}?{}{}&api_key={}&file_type=json", //&limit=10000",
                path,
                q,
                i,
                finox::keys::FRED_KEY
            );
            let url = r.join(&q).expect("url parsed wrong");
            println!("{:#?}", url);

            urls.push(url);
        }
    }
    return urls;
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CategoryRoot {
    pub realtime_start: String,
    pub realtime_end: String,
    pub order_by: String,
    pub sort_order: String,
    pub count: i64,
    pub offset: i64,
    pub limit: i64,
    pub seriess: Vec<Series>,
}

impl CategoryRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        return self.seriess.iter().map(|x| x.to_rec()).collect::<Vec<_>>();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Series {
    pub id: String,
    pub realtime_start: String,
    pub realtime_end: String,
    pub title: String,
    pub observation_start: String,
    pub observation_end: String,
    pub frequency: String,
    pub frequency_short: String,
    pub units: String,
    pub units_short: String,
    pub seasonal_adjustment: String,
    pub seasonal_adjustment_short: String,
    pub last_updated: String,
    pub popularity: i64,
    pub group_popularity: i64,
    pub notes: String,
}

impl Series {
    fn to_rec(&self) -> Vec<String> {
        return vec![
            self.id.to_string(),
            self.realtime_start.to_string(),
            self.realtime_end.to_string(),
            self.title.to_string(),
            self.observation_start.to_string(),
            self.observation_end.to_string(),
            self.frequency_short.to_string(),
            self.units_short.to_string(),
            self.seasonal_adjustment_short.to_string(),
            self.last_updated.to_string(),
            self.popularity.to_string(),
            self.group_popularity.to_string(),
            self.notes.to_string(), // fix to just grab source code
        ];
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SeriesObsRoot {
    pub realtime_start: String,
    pub realtime_end: String,
    pub observation_start: String,
    pub observation_end: String,
    pub units: String,
    pub output_type: i64,
    pub file_type: String,
    pub order_by: String,
    pub sort_order: String,
    pub count: i64,
    pub offset: i64,
    pub limit: i64,
    pub observations: Vec<Observation>,
}

impl SeriesObsRoot{
    fn to_recs(&self) -> Vec<Vec<String>> {
        return self.observations.iter().map(|x| x.to_rec()).collect::<Vec<_>>();
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Observation {
    pub realtime_start: String,
    pub realtime_end: String,
    pub date: String,
    pub value: String,
}

impl Observation {
    fn to_rec(&self) -> Vec<String> {
        return vec![
            self.date.to_string(),
            self.value.to_string()
        ];
    }
}

pub const OBS_HEADER: [&'static str; 2] = [
    "date", 
    "value"
];


pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub const SERIES_HEADER: [&'static str; 13] = [
    "id",
    "realtime_start",
    "realtime_end",
    "title",
    "observation_start",
    "observation_end",
    "frequency_short",
    "units_short",
    "seasonal_adjustment_short",
    "last_updated",
    "popularity",
    "group_popularity",
    "notes",
];
