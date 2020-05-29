#[allow(dead_code)]
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate url;

//use futures::stream::StreamExt;
//use lazy_static::lazy_static;
//use regex::Regex;
use std::{env, path::Path, time};
use url::Url;
pub const WRITE_PATH: &str = "../data/fred/";
pub const DELAY: time::Duration = time::Duration::from_millis(500);

//fn extract_id(input: &str) -> Option<&str> {
//    lazy_static! {
//        static ref RE: Regex = Regex::new(r"id=(?P<id>\w*)&").unwrap();
//    }
//    RE.captures(input)
//        .and_then(|cap| cap.name("id").map(|login| login.as_str()))
//
//}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "provide 's' or 'o' as second arg, for 'o' you need a fred_series_id.txt in ref_data/"
        );
    }
    // how do you 
    match args[1].as_ref() {
        "s" => {
            let urls = gen_queries("category/series", "category_id=", (0..1000).into_iter().map(|x| x.to_string()).collect())
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if let Ok(recs) = finox::fetch::<CategoryRoot>(urls).await {
                let file_name = format!(
                    "../data/fred/fred_series_{}.csv",
                    chrono::Utc::now().to_rfc3339()
                );
                let file_path = Path::new(&file_name);
                roses::write_csv(file_path, recs, &SERIES_HEADER).expect("csv prob");
            }
        },
        "o" => {
            let ids = roses::read_tickers("../ref_data/fred_series_ids.txt");
            let urls = gen_queries("series/observations", "series_id=", ids)
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if let Ok(recs) = finox::fetch::<SeriesObsRoot>(urls.to_vec()).await {
                let file_name = format!(
                    "../data/fred/fred_obs_{}.csv",
                    chrono::Utc::now().to_rfc3339()
                );
                let file_path = Path::new(&file_name);
                roses::write_csv(file_path, recs, &OBS_HEADER).expect("csv prob");
            }
        },
        _ => panic!("'s' or 'o' for series or observation as 2nd command line arg"),
    };
    Ok(())
}

fn gen_queries(
    //ids: Vec<String>,
    path: &str,
    q: &str,
    ids: Vec<String> )
 -> Vec<url::Url> {
    //TODO FIX
    ids.iter().map(|x| fred_fmt(path, q, x)).collect::<Vec<url::Url>>()
}

fn fred_fmt(path: &str, query: &str, id: &str) -> Url {
        let root = Url::parse("https://api.stlouisfed.org").expect("url prob");
        let q = format!(
                "fred/{}?{}{}&api_key={}&file_type=json", //&limit=10000",
                path,
                query,
                id,
                finox::keys::FRED_KEY
            );
            let url = root.join(&q).expect("url parsed wrong");
            println!("{:#?}", url);
            return url;
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

impl finox::HasRecs for CategoryRoot {
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

impl finox::HasRecs for SeriesObsRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        return self
            .observations
            .iter()
            .map(|x| x.to_rec())
            .collect::<Vec<_>>();
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
        return vec![self.date.to_string(), self.value.to_string()];
    }
}

pub const OBS_HEADER: [&'static str; 2] = ["t", "x"];

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
