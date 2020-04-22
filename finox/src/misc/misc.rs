
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




