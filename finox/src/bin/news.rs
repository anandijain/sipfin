extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use finox;
use std::path::Path;

#[tokio::main]
pub async fn main() {
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
        //println!("{:#?}", recs);
        let file_name = format!("../data/news/wsj_{}.csv", chrono::Utc::now().to_rfc3339());
        let file_path = Path::new(&file_name);
        roses::write_csv(file_path, recs, &finox::headers::WSJ_HEADER).expect("csv prob");
    }

    let gs_url = "https://www.goldmansachs.com/insights/insights-articles.json";
    if let Ok(recs) = finox::fetch::<finox::news::gs::Root>(vec![gs_url.to_string()]).await {
        println!("goldman {:#?}", recs);
        let file_name = format!("../data/news/gs_{}.csv", chrono::Utc::now().to_rfc3339());
        let file_path = Path::new(&file_name);

        roses::write_csv(file_path, recs, &finox::headers::GS_HEADER).expect("csv problem");
    }

    let moodys_url = "https://www.moodys.com/_layouts/mdc/am/Request/request.php?profile=homepage"; // "https://www.goldmansachs.com/insights/insights-articles.json";
    if let Ok(recs) = finox::fetch::<finox::news::moodys::Root>(vec![moodys_url.to_string()]).await {
        println!("goldman {:#?}", recs);
        let file_name = format!("../data/news/moodys_{}.csv", chrono::Utc::now().to_rfc3339());
        let file_path = Path::new(&file_name);

        roses::write_csv(file_path, recs, &finox::headers::MOODYS_HEADER).expect("csv problem");
    }


    // TODO fix, serializing err
    //let sa_url = "https://seekingalpha.com/get_trending_articles".to_string();

    //if let Ok(recs) = finox::fetch::<finox::news::sa::SARoot>(vec![sa_url]).await {
    //    println!("{:#?}", recs);
    //    let file_name = format!("../data/news/sa_{}.csv", chrono::Utc::now().to_rfc3339());
    //    let file_path = Path::new(&file_name);
    //    roses::write_csv(file_path, recs, &finox::headers::SA_HEADER).expect("csv prob");
    //}
    //bloomberg::news();
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
