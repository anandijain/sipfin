
// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Collections {
//     pub field_data_collection: Vec<FieldDataCollection>,
// }

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FieldDataCollection {
//     pub id: String,
//     pub issued_currency: String,
//     pub long_name: String,
//     pub price: String,
//     pub price_change1_day: String,
//     pub percent_change1_day: String,
//     #[serde(rename = "tradingDayCloseUTC")]
//     pub trading_day_close_utc: String,
//     #[serde(rename = "lastUpdateUTC")]
//     pub last_update_utc: String,
//     #[serde(rename = "MEDIA_SECURITY_TYPE")]
//     pub media_security_type: String,
//     #[serde(rename = "MEDIA_SECURITY_SUBTYPE")]
//     pub media_security_subtype: String,
//     pub security_type: String,
//     pub short_name: String,
//     pub commodity_contract_date: String,
//     pub price_date: String,
//     pub last_update_time: String,
//     #[serde(rename = "lastUpdateISO")]
//     pub last_update_iso: String,
//     pub user_time_zone: String,
//     pub market_open: bool,
//     pub commodity_units: Option<String>,
// }

// pub fn hs_and_st() -> Result<(), reqwest::Error> {
//     let url = "https://comtrade.un.org/Data/cache/classificationST.json";
//     let write_fn = "st.csv";
//     //  "https://comtrade.un.org/Data/cache/classificationST.json"];
//     // for url in urls.iter() {
//     if let Ok(body) = getters::simple_get(url.to_string()) {
//         let res: uncomtrade::ResMeta = serde_json::from_str(&body.to_string()).unwrap();
//         let recs = uncomtrade::ResMeta::to_records(&res);
//         writerecs(write_fn.to_string(), &["id", "text", "parent"], recs);
//     }
//     Ok(())
// }





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
