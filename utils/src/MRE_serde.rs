use futures::stream::StreamExt;
use serde_json::{Deserialize, Serialize};

/*
 * working on MRE for serde traits
 */

//fn main() -> Result<(), String> {
//	
//}
//
//pub async fn fetch<T>(urls: 'de Vec<String>) -> Vec<Option<'de Vec<Vec<String>>>>
//where
//    T: HasRecs + 'de Deserialize,
//{
//    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
//        if let Ok(res) = reqwest::get(&url).await {
//            if let Ok(root) = res.json::<T, 'de>() {
//                return Some(root.to_recs());
//            } else {
//                println!("serialize err {}", url.clone());
//                return None;
//            }
//        }
//        println!("response err{}", url.clone());
//        return None;
//    }))
//    .buffer_unordered(16)
//    .collect::<Vec<Option<Vec<Vec<String>>>>>()
//    .await;
//
//    return fetches;
//}
//
//pub trait Records {
//    fn to_recs(&self) -> Vec<Vec<String>>;
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct NewsArticle {
//    pub headline: String,
//    pub location: String,
//    pub author: String,
//    pub content: String,
//}
//
//impl Records for NewsArticle {
//    fn to_recs(&self) -> Vec<Vec<String>> {
//        vec![self.headline.to_string(), self.author.to_string()];
//    }
//}
//
//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct NewsArticle2 {
//    pub headline_text: String,
//    pub source: String,
//    pub author: String,
//}
//
//impl Summary for Tweet {
//    fn to_recs(&self) -> Vec<Vec<String>> {
//        vec![self.headline_text.to_string(), self.author.to_string()];
//    }
//}
