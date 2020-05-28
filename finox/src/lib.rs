pub mod headers;
pub mod keys;
pub mod news;
pub mod yf;
pub mod cme;

use std::{time::Duration};
use futures::stream::StreamExt;

pub trait HasRecs {
    fn to_recs(&self) -> Vec<Vec<String>>;
}

pub const NYT_DELAY: Duration = Duration::from_millis(6000);

pub async fn fetch<'a, T: ?Sized>(urls: Vec<String>) -> Result<Vec<Vec<String>>, String>
where
    for<'de> T: HasRecs + serde::Deserialize<'de> + 'a,
{
    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url).await {
            if let Ok(root) = res.json::<T>().await {
                return Some(root.to_recs());
            } else {
                println!("serialize err {}", url.clone());
                return None;
            }
        }
        println!("response err: {}", url.clone());
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<Vec<Vec<String>>>>>()
    .await;
    let recs = fetches
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<Vec<String>>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Vec<String>>>();
    //Ok(Box::new(fetches.into_iter().flatten().collect::<Vec<T>>()))
    Ok(recs)
}

// when endpoints dont grab a vec
//pub async fn fetch_rec(urls: Vec<String>) -> Vec<Vec<String>> {
//    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
//        if let Ok(res) = reqwest::get(&url.clone()).await {
//            if let Ok(root) = res.json::<crate::nasdaq::info::InfoRoot>().await {
//                return Some(root.to_rec());
//            }
//            println!("serialized json wrong {}", url.clone());
//            return None;
//        }
//        println!("no good1");
//        return None;
//    }))
//    .buffer_unordered(16)
//    .collect::<Vec<Option<Vec<String>>>>()
//    .await;
//    let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
//    return recs;
//}


pub fn gsnews() -> Result<(), reqwest::Error> {
    let url = "https://www.goldmansachs.com/insights/insights-articles.json";
    if let Ok(body) = roses::simple_get(url.to_string()) {
        let root: news::gs::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::gs::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        roses::writerecs("./gsnews.csv".to_string(), &headers::GS_HEADER, recs)
            .expect("csv problem");
    }
    Ok(())
}
pub fn nyt_archive_urls() -> Vec<String> {
    let mut urls = vec![];
    for i in 1853..2019 {
        for j in 1..=12 {
            let url = format!(
                "https://api.nytimes.com/svc/archive/v1/{}/{}.json?api-key={}",
                i,
                j,
                crate::keys::NYT_KEY.to_string()
            );
            urls.push(url);
        }
    }
    urls
}

