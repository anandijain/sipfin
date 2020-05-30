//extern crate regex;
extern crate reqwest;
extern crate roses;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate url;
//use lazy_static::lazy_static;
//use regex::Regex;

use finox::keys::FRED_KEYS;
use std::{
    collections::HashMap,
    env,
    path::Path,
    //thread,
    //rc::{Rc, Weak},
    time,
};
use url::Url;

pub const WRITE_PATH: &str = "../data/fred/";
pub const FRED_DELAY: time::Duration = time::Duration::from_secs(10);

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
        "c" => {
            /*
             * 1. for bfs we want
             */
            //let mut cat_ids = vec![];
            let path = "category/children";
            let query = "category_id=";
            let mut all_recs: Vec<Vec<String>> = vec![];

            let depth: usize = 4;

            let mut to_visit: Vec<String> = vec![
                "32991", "10", "32992", "1", "32455", "32263", "3008", "33060",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect();
            //for _ in 0..depth {
            //    //let id = to_visit.pop().unwrap();
            //    if let Ok(recs) = finox::fetch::<CategoryChildrenRoot>(
            //        gen_queries(path, query, to_visit.clone())
            //            .values()
            //            .collect::<Vec<String>>(),
            //    )
            //    .await
            //    {
            //        all_recs.append(&mut recs.clone());
            //        to_visit = recs
            //            .iter()
            //            .map(|x| x.clone()[0].to_string())
            //            .collect::<Vec<String>>();

            //        println!("recs: {:#?}", recs);
            //        println!("to_visit: {:#?}", to_visit);
            //    }
            //}

            //println!("all recs: {:#?}\nlen: {}", all_recs, all_recs.len());
            //roses::write_csv(
            //    Path::new("../data/fred/categories.csv"),
            //    all_recs,
            //    &CATEGORY_HEADER,
            //)
            //.expect("csv prob");
        }

        "s" => {
            let ids = roses::read_tickers("../ref_data/fred_category_ids.txt");
            let urls = gen_queries("category/series", "category_id=", ids);
            let res =
                finox::fetch_write::<CategoryRoot>(urls, "../data/fred/series/", &SERIES_HEADER)
                    .await;
            println!("{:#?}", res);
        }
        "o" => {
            let ids = roses::read_tickers("../ref_data/fred_series_ids.txt");
            let urls = gen_queries("series/observations", "series_id=", ids);
            let res = finox::fetch_write::<SeriesObsRoot>(
                urls,
                "../data/fred/observations/",
                &OBS_HEADER,
            )
            .await;
            println!("{:#?}", res);
        }
        _ => panic!("'s' or 'o' for series or observation as 2nd command line arg"),
    };
    Ok(())
}

fn gen_queries(path: &str, q: &str, ids: Vec<String>) -> HashMap<String, String>
//-> Vec<url::Url>
{
    let mut queries = HashMap::new();
    for (i, id) in ids.iter().enumerate() {
        queries.insert(
            id.to_string(),
            fred_fmt(path, q, id, FRED_KEYS[i % FRED_KEYS.len()]).to_string(),
        );
        //.collect::<Vec<url::Url>>()
    }
    queries
}

fn fred_fmt(path: &str, query: &str, id: &str, key: &str) -> Url {
    let root = Url::parse("https://api.stlouisfed.org").expect("url prob");

    let q = format!(
        "fred/{}?{}{}&api_key={}&file_type=json", //&limit=10000",
        path, query, id, key,
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

pub const CATEGORY_HEADER: [&'static str; 2] = ["id", "name"];

pub const OBS_HEADER: [&'static str; 2] = ["t", "x"];

pub const SERIES_HEADER: [&'static str; 11] = [
    "id",
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CategoryChildrenRoot {
    pub categories: Vec<CategoryChild>,
}

impl finox::HasRecs for CategoryChildrenRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.categories.iter().map(|x| x.to_rec()).collect()
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CategoryChild {
    pub id: i64,
    pub name: String,
    pub parent_id: i64,
    pub notes: Option<String>,
}

impl CategoryChild {
    pub fn to_rec(&self) -> Vec<String> {
        vec![self.id.to_string(), self.name.to_string()]
    }
}
