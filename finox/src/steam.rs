extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::collections::HashMap;


use crate::utils;
/*

https://steamcommunity.com/market/recent?country=US&language=english&currency=1 new listings
https://steamcommunity.com/market/recentcompleted

https://steamcommunity.com/market/itemordersactivity?country=US&language=english&currency=1&item_nameid=1
*/

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steam {
    pub success: bool,
    pub more: bool,
    #[serde(rename = "results_html")]
    pub results_html: ::serde_json::Value,
    pub listinginfo: HashMap<String, Listing>, //Listings
    pub purchaseinfo: Option<HashMap<String, Listing>>, //Purchases
    pub assets: Games,
    pub currency:  ::serde_json::Value,
    pub hovers: String,
    #[serde(rename = "app_data")]
    pub app_data: AppDatas,
    #[serde(rename = "last_time")]
    pub last_time: i64,
    #[serde(rename = "last_listing")]
    pub last_listing: String,
}

impl Steam {
    pub fn listings(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for (k, v) in self.listinginfo.iter() {
            recs.push(Listing::to_record(v));
        } 
        return recs;
    }
    pub fn purchases(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for (k, v) in self.listinginfo.iter() {
            recs.push(Listing::to_record(v));
        } 
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listings {
    #[serde(flatten)]
    listings: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub listingid: String,
    pub price: Option<i64>,
    pub fee: Option<i64>,
    #[serde(rename = "publisher_fee_app")]
    pub publisher_fee_app: Option<i64>,
    #[serde(rename = "publisher_fee_percent")]
    pub publisher_fee_percent: String,
    pub currencyid: ::serde_json::Value,
    #[serde(rename = "steam_fee")]
    pub steam_fee: Option<i64>,
    #[serde(rename = "publisher_fee")]
    pub publisher_fee: Option<i64>,
    #[serde(rename = "converted_price")]
    pub converted_price: Option<i64>,
    #[serde(rename = "converted_fee")]
    pub converted_fee: Option<i64>,
    #[serde(rename = "converted_currencyid")]
    pub converted_currencyid: Option<i64>,
    #[serde(rename = "converted_steam_fee")]
    pub converted_steam_fee: Option<i64>,
    #[serde(rename = "converted_publisher_fee")]
    pub converted_publisher_fee: Option<i64>,
    #[serde(rename = "converted_price_per_unit")]
    pub converted_price_per_unit: Option<i64>,
    #[serde(rename = "converted_fee_per_unit")]
    pub converted_fee_per_unit: Option<i64>,
    #[serde(rename = "converted_steam_fee_per_unit")]
    pub converted_steam_fee_per_unit: Option<i64>,
    #[serde(rename = "converted_publisher_fee_per_unit")]
    pub converted_publisher_fee_per_unit: Option<i64>,
    pub asset: Asset,
}

impl Listing {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec = vec!(
            self.listingid.to_string(),
            utils::lilmatcher_i64(self.fee.clone()),
            utils::lilmatcher_i64(self.price.clone()),
            utils::lilmatcher_i64(self.publisher_fee_app.clone()),
            self.publisher_fee_percent.clone(),
            self.currencyid.to_string(),
        );
        rec.append(&mut Asset::to_record(&self.asset));
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub currency: Option<i64>,
    pub appid: Option<i64>,
    pub contextid: Option<String>,
    pub id: Option<String>,
    pub classid: Option<String>,
    pub instanceid: Option<String>,
    pub amount: Option<String>,
    pub status: Option<i64>,
    #[serde(rename = "original_amount")]
    pub original_amount: Option<String>,
    #[serde(rename = "unowned_id")]
    pub unowned_id: Option<String>,
    #[serde(rename = "unowned_contextid")]
    pub unowned_contextid: Option<String>,
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    #[serde(rename = "icon_url_large")]
    pub icon_url_large: Option<String>,
    pub descriptions: Option<Vec<Description>>,
    pub tradable: Option<i64>,
    #[serde(rename = "owner_actions")]
    pub owner_actions: Option<Vec<OwnerAction>>,
    pub name: Option<String>,
    pub name_color: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "market_name")]
    pub market_name: Option<String>,
    #[serde(rename = "market_hash_name")]
    pub market_hash_name: Option<String>,
    #[serde(rename = "market_fee_app")]
    pub market_fee_app: Option<i64>,
    pub commodity: Option<i64>,
    #[serde(rename = "market_tradable_restriction")]
    pub market_tradable_restriction: Option<i64>,
    #[serde(rename = "market_marketable_restriction")]
    pub market_marketable_restriction: Option<i64>,
    #[serde(rename = "market_actions")]
    pub market_actions:  Option<::serde_json::Value>,
    pub marketable: Option<i64>,
    #[serde(rename = "app_icon")]
    pub app_icon: Option<String>,
    pub owner: Option<i64>,
    #[serde(rename = "new_id")]
    pub new_id: Option<String>,
    #[serde(rename = "new_contextid")]
    pub new_contextid: Option<String>,
}

impl Asset {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            utils::lilmatcher(self.id.clone()),
            utils::lilmatcher(self.name.clone()),
            utils::lilmatcher_i64(self.appid.clone()),
            utils::lilmatcher(self.amount.clone()),
            utils::lilmatcher_i64(self.status.clone()),
            utils::lilmatcher_i64(self.currency.clone()),
            utils::lilmatcher_i64(self.tradable.clone()),
        );
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
    pub color: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerAction {
    pub link: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    #[serde(flatten)]
    games: HashMap<String, AssetMap>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetMap {
    #[serde(flatten)]
    assetmap: HashMap<String, Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Purchase {
    pub listingid: String,
    pub purchaseid: String,
    #[serde(rename = "paid_amount")]
    pub paid_amount: i64,
    #[serde(rename = "paid_fee")]
    pub paid_fee: i64,
    pub currencyid: String,
    #[serde(rename = "steam_fee")]
    pub steam_fee: i64,
    #[serde(rename = "publisher_fee")]
    pub publisher_fee: i64,
    #[serde(rename = "publisher_fee_percent")]
    pub publisher_fee_percent: String,
    #[serde(rename = "publisher_fee_app")]
    pub publisher_fee_app: i64,
    pub asset: Asset,
}

impl Purchase {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec = vec!(
            self.listingid.to_string(),
            self.purchaseid.to_string(),
            self.paid_amount.to_string(),
            self.paid_fee.to_string(),
            self.steam_fee.to_string(),
            self.publisher_fee.to_string(),
            self.publisher_fee_app.to_string(),
            self.publisher_fee_percent.to_string(),
        );
        rec.append(&mut Asset::to_record(&self.asset));
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamAppData {
    pub appid: i64,
    pub name: String,
    pub icon: String,
    pub link: String,
}

impl SteamAppData {
    pub fn to_record(&self) -> Vec<String> {
        let rec = vec!(
            self.appid.to_string(),
            self.name.to_string(),
        );
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDatas {
    #[serde(flatten)]
    gamedata: HashMap<String, SteamAppData>,
}

// pub const STEAM_ASSET_HEADER: [&'static str; 6] = [];
pub const STEAM_LISTING_HEADER: [&'static str; 6] = [
    "l_id",
    "l_price",
    "l_fee",
    "l_pub_fee_app",
    "l_pub_fee_pct",
    "l_currency_id",
];

// pub const STEAM_ASSET_HEADER: [&'static str; 6] = [];
pub const STEAM_PURCHASE_HEADER: [&'static str; 8] = [
    "p_listing_id",
    "p_id",
    "p_paid_amt",
    "p_paid_fee",
    "p_cur_id",
    "p_steam_fee",
    "p_pub_fee",
    "p_pub_fee_pct",
];

pub const STEAM_ASSET_HEADER: [&'static str; 7] = [
    "a_id",
    "a_name",
    "a_appid",
    "a_amount",
    "a_status",
    "a_currency",
    "a_tradable",
];

// pub const STEAM_ASSET_HEADER: [&'static str; 6] = [];
pub const STEAM_PURCHASE_HEADER2: [&'static str; 15] = [
    "p_listing_id",
    "p_id",
    "p_paid_amt",
    "p_paid_fee",
    "p_cur_id",
    "p_steam_fee",
    "p_pub_fee",
    "p_pub_fee_pct",
    "a_id",
    "a_name",
    "a_appid",
    "a_amount",
    "a_status",
    "a_currency",
    "a_tradable",
];
// pub const STEAM_PURCHASE_HEADER: [&'static str; 6] = [];


