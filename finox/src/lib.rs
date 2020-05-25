pub mod headers;
pub mod yf;
pub mod keys;
pub mod news;
//use std::{thread, time::Duration};

pub fn sa() -> Result<(), reqwest::Error> {
    let url = "https://seekingalpha.com/get_trending_articles";
    if let Ok(body) = roses::simple_get(url.to_string()) {
        let root: news::sa::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::sa::Root::to_recs(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        roses::writerecs("./sa.csv".to_string(), &headers::SA_HEADER, recs).expect("csv prob");
    }
    Ok(())
}

//pub fn reuters() -> Result<(), csv::Error> {
//    let file_name = "./reuters.csv".to_string();
//    let mut wtr = csv::Writer::from_path(file_name.to_string())?;
//    wtr.write_record(&headers::REUTERS_HEADER);
//    for country in headers::REUTERS_COUNTRIES.iter() {
//        let url = format!("https://sope.prod.reuters.tv/program/rcom/v1/article-recirc?edition={}&modules=rightrail,ribbon,bottom", country.to_string());
//        if let Ok(body) = roses::simple_get(url.to_string()) {
//            let root: news::TR = serde_json::from_str(&body.to_string()).unwrap();
//            let recs: Vec<csv::StringRecord> = news::TR::to_records(&root)
//                .into_iter()
//                .map(|x| csv::StringRecord::from(x))
//                .collect();
//            for r in recs.iter() {
//                wtr.write_record(r);
//            }
//        }
//    }
//    wtr.flush();
//    Ok(())
//}

//pub fn wsj_videos() {
//    let url = "https://video-api.wsj.com/api-video/find_all_videos.asp";
//    if let Ok(body) = roses::simple_get(url.to_string()) {
//        let root: news::WSJ = serde_json::from_str(&body.to_string()).unwrap();
//        let recs = news::WSJ::to_records(&root)
//            .into_iter()
//            .map(|x| csv::StringRecord::from(x))
//            .collect();
//        roses::
//    }
//}

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
    if let Ok(body) = roses::simple_get(url.to_string()) {
        let root: news::gs::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs = news::gs::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();
        roses::writerecs("./gsnews.csv".to_string(), &headers::GS_HEADER, recs).expect("csv problem");
    }
    Ok(())
}

//pub fn nytfeed() -> Result<(), reqwest::Error> {
//    let url = format!(
//        "https://api.nytimes.com/svc/news/v3/content/all/all.json?api-key={}&limit=200",
//        crate::keys::NYT_KEY.to_string()
//    );
//    if let Ok(body) = roses::simple_get(url.to_string()) {
//        let root: news::NYTFeed = serde_json::from_str(&body.to_string()).unwrap();
//        let recs = news::NYTFeed::to_records(&root)
//            .into_iter()
//            .map(|x| csv::StringRecord::from(x))
//            .collect();
//        roses::writerecs("./nytfeed.csv".to_string(), &headers::NYT_FEED_HEADER, recs);
////use headers;
//    }
//    Ok(())
//}
//
//pub fn nytarchive() -> Result<(), csv::Error> {
//    let filename = "./nyt_archive.csv".to_string();
//    let mut wtr = csv::Writer::from_path(filename)?;
//    let NYT_DELAY: std::time::Duration = Duration::from_millis(6000);
//
//    wtr.write_record(&headers::NYT_ARCHIVE_HEADER);
//    for i in 1853..2019 {
//        for j in 1..13 {
//            let url = format!(
//                "https://api.nytimes.com/svc/archive/v1/{}/{}.json?api-key={}",
//                i,
//                j,
//                crate::keys::NYT_KEY.to_string()
//            );
//            if let Ok(body) = roses::simple_get(url.to_string()) {
//                let root: news::NYTArchive =
//                    serde_json::from_str(&body.to_string()).unwrap();
//                let recs: Vec<csv::StringRecord> = news::NYTArchive::to_records(&root)
//                    .into_iter()
//                    .map(|x| csv::StringRecord::from(x))
//                    .collect();
//                for r in recs.iter() {
//                    wtr.write_record(r);
//                }
//                thread::sleep(NYT_DELAY);
//            }
//        }
//    }
//    wtr.flush();
//    Ok(())
//}
