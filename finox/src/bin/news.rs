extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use finox;
use std::path::Path;

#[tokio::main]
pub async fn main() -> Result<(), csv::Error> {
    // utils::nytarchive();

    //finox::nytfeed()?;
    //finox::gsnews()?;
    //guardian_news().await?;
    // TODO: do async and collect HashMap<String, Vec<Vec<String>>>
    if let Ok(recs) =
        finox::fetch::<finox::news::guardian::ArticleRoot>(vec![urlfmt("search")]).await
    {
        println!("{:#?}", recs);
        let file_name = format!(
            "../data/news/guardian_{}.csv",
            chrono::Utc::now().to_rfc3339()
        );
        let file_path = Path::new(&file_name);
        roses::write_csv(file_path, recs, &finox::headers::GUARDIAN_HEADER).expect("csv prob");
    }

    let nyt_url = format!(
        "https://api.nytimes.com/svc/news/v3/content/all/all.json?api-key={}&limit=200",
        finox::keys::NYT_KEY.to_string()
    );

    if let Ok(recs) = finox::fetch::<finox::news::nyt::NYTFeed>(vec![nyt_url]).await {
        //println!("{:#?}", recs);
        let file_name = format!("../data/news/nyt_{}.csv", chrono::Utc::now().to_rfc3339());
        let file_path = Path::new(&file_name);
        roses::write_csv(file_path, recs, &finox::headers::NYT_FEED_HEADER).expect("csv prob");
    }

    // jpx wont work because root is vec, prob a flatten fix
    //let jpx_url = "https://www.jpx.co.jp/english/news/news_ym_01.json";

    //if let Ok(recs) = finox::fetch::<finox::news::jpxnews::JPX>(vec![jpx_url]).await {
    //     println!("{:#?}", recs);
    //}
    let wsj_url = "https://video-api.wsj.com/api-video/find_all_videos.asp".to_string();

    if let Ok(recs) = finox::fetch::<finox::news::wsj::WSJRoot>(vec![wsj_url]).await {
        println!("{:#?}", recs);
        let file_name = format!("../data/news/wsj_{}.csv", chrono::Utc::now().to_rfc3339());
        let file_path = Path::new(&file_name);
        roses::write_csv(file_path, recs, &finox::headers::WSJ_HEADER).expect("csv prob");
    }

    // TODO fix, serializing err
    //let sa_url = "https://seekingalpha.com/get_trending_articles".to_string();

    //if let Ok(recs) = finox::fetch::<finox::news::sa::SARoot>(vec![sa_url]).await {
    //    println!("{:#?}", recs);
    //    let file_name = format!("../data/news/sa_{}.csv", chrono::Utc::now().to_rfc3339());
    //    let file_path = Path::new(&file_name);
    //    roses::write_csv(file_path, recs, &finox::headers::SA_HEADER).expect("csv prob");
    //}
    //finox::wsj_videos();
    //bloomberg::news();
    Ok(())
}

//pub async fn fetch_write_blocking<T: finox::HasRecs>(urls: Vec<String>) -> Result<(), csv::Error> {
//    if let Ok(recs) = finox::fetch::<T>(urls).await {
//        println!("{:#?}", recs);
//        let file_name = format!("../data/news/wsj_{}.csv", chrono::Utc::now().to_rfc3339());
//        let file_path = Path::new(&file_name);
//        roses::write_csv(file_path, recs, &finox::headers::WSJ_HEADER).expect("csv prob");
//    }
//}

pub fn urlfmt(s: &str) -> String {
    format!(
        "https://content.guardianapis.com/{}?api-key={}",
        s,
        finox::keys::GUARDIAN_KEY
    )
}

//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct NewsVec {
//    pub news: Vec<News>,
//}
//
//impl NewsVec {
//    pub fn to_records(&self) -> Result<Vec<csv::StringRecord>, csv::Error> {
//        let mut ret: Vec<csv::StringRecord> = Vec::new();
//        for article in self.news.iter() {
//            ret.push(News::to_record(article));
//        }
//        Ok(ret)
//    }
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct News {
//    pub headline: String,
//    pub published_at: String,
//    pub url: String,
//    #[serde(rename = "publishedAtISO")]
//    pub published_at_iso: String,
//}
//
//impl News {
//    pub fn to_record(&self) -> csv::StringRecord {
//        let hl_text = self.headline.replace(",", ";");
//        let rec = &[
//            self.url.to_string(),
//            hl_text.to_string(),
//            self.published_at.to_string(),
//        ];
//        return csv::StringRecord::from(rec.to_vec());
//    }
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct Channel {
//    pub path: String,
//    pub name: String,
//}
