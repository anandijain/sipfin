extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::getters;
use crate::news;
use crate::utils;
/*
todo
fn dash() -> Result<(), reqwest::Error> {
let symbs = vec!["commodities", "futures", "asia", "americas", "europe"];
let dashboard_root = "https://www.bloomberg.com/markets/api/data-dashboard/tileset/";
https://www.bloomberg.com/markets2/api/report/income/EQT/MSFT%3AUS/annual?locale=en&currency=USD
https://www.bloomberg.com/markets/api/security/currency/cross-rates/USD,EUR
https://www.bloomberg.com/markets2/api/people/2029055
https://www.bloomberg.com/markets2/api/peopleForCompany/101743
https://www.bloomberg.com/markets/api/sectors/S5INFT%3AIND?locale=en
https://www.bloomberg.com/markets2/api/history/MSFT%3AUS/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily
https://www.bloomberg.com/markets2/api/history/CL1%3ACOM/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily
https://www.bloomberg.com/markets/api/comparison/news?securityType=GOVERNMENT_BOND&limit=1000&locale=en
anotha SP1:IND,DM1:IND,SX5E:IND,UKX:IND,DAX:IND,NKY:IND,SHCOMP:IND,SPX:IND,RTY:IND,DXY:CUR,USDJPY:CUR,EURUSD:CUR,XAU:CUR,USGG10YR:IND,USGG2YR:IND,LEGATRUU:IND,CL1:COM,CO1:COM
https://www.bloomberg.com/bbg-gfx/bgreen-widget-data/dashboard-data.json
https://oec.world/en/profile/country/arg/
https://api.nasdaq.com/api/quote/watchlist?symbol=cl%3anmx%7ccommodities&symbol=ho%3anmx%7ccommodities&symbol=rb%3anmx%7ccommodities&symbol=ng%3anmx%7ccommodities&symbol=bz%3anmx%7ccommodities&symbol=eh%7ccommodities
https://www.bloomberg.com/markets/api/security/currency/cross-rates/USD,EUR,XAU,XAG,XPT,XPD,JPY,GBP,AUD,CAD,CHF,KRW,MXN,BRL,CLP,COP,PEN,CRC,ARS,SEK,DKK,NOK,CZK,SKK,PLN,HUF,RUB,TRY,ILS,KES,ZAR,MAD,NZD,PHP,SGD,IDR,CNY,INR,MYR,THB,
}
*/

pub const CURRENCY_SYMBOLS: [&'static str; 40] = [
    "USD", "EUR", "XAU", "XAG", "XPT", "XPD", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN",
    "BRL", "CLP", "COP", "PEN", "CRC", "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF",
    "RUB", "TRY", "ILS", "KES", "ZAR", "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR",
    "THB",
];
// USD,EUR,XAU,XAG,XPT,XPD,JPY,GBP,AUD,CAD,CHF,KRW,MXN,BRL,CLP,COP,PEN,CRC,ARS,SEK,DKK,NOK,CZK,SKK,PLN,HUF,RUB,TRY,ILS,KES,ZAR,MAD,NZD,PHP,SGD,IDR,CNY,INR,MYR,THB,
pub const NEWS_SYMBOLS: [&'static str; 5] = [
    "GOVERNMENT_BOND",
    "COMMODITY",
    "COMMON_STOCK",
    "CURRENCY",
    "BLOOMBERG_BARCLAYS_INDEX",
];

pub const COMMODITIES_SYMBOLS: [&'static str; 37] = [
    "CO1", "CL1", "XB1", "NG1", "HO1", "GC1", "SI1", "HG1", "C%201", "W%201", "CC1", "CT1", "LC1",
    "QS1", "JX1", "MO1", "JG1", "LMCADS03", "LMAHDS03", "LMZSDS03", "LMSNDS03", "O%201", "RR1",
    "S%201", "SM1", "BO1", "RS1", "KC1", "SB1", "JO1", "CT1", "OL1", "LB1", "JN1", "DL1", "FC1",
    "LH1",
];

pub const STOCK_HEADER: [&'static str; 15] = [
    "id",
    "short_name",
    "market_cap",
    "co_phone",
    "last_update",
    "average_volume30_day",
    "price",
    "open_price",
    "high_price",
    "low_price",
    "low_price52_week",
    "high_price52_week",
    "number_of_employees",
    "price_earnings_ratio",
    "shares_outstanding",
];

// pub fn currencies_intraday(start: String) -> Result<(), reqwest::Error> {
//     let urls = currency_urls();
//     for url in urls.iter() {
//         if let Some(curs) = get_intraday_or_history(url.to_string()) {
//             let prices_fn = format!("./data/{}_intraday_prices.csv", url.to_string());
//             if let Ok(recs) = Intraday::price_records(&curs[0]) {
//                 utils::writerecs(prices_fn, &["date_time", &curs[0].ticker.to_string()], recs);
//             }
//         } else {
//             println!("currency route missing: {}", url.to_string());
//             continue;
//         }
//     }
//     Ok(())
// }

// pub fn currencies_history(start: String) -> Result<(), reqwest::Error> {
//     let index = CURRENCY_SYMBOLS
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &CURRENCY_SYMBOLS[index..CURRENCY_SYMBOLS.len()];
//     for s1 in todo_symbs.iter() {
//         for s2 in CURRENCY_SYMBOLS.iter() {
//             if s1 == s2 {
//                 continue;
//             }
//             let symb = format!("{}{}%3ACUR", s1.to_string(), s2.to_string());
//             if let Some(curs) = get_history(symb.to_string()) {
//                 let prices_fn = format!("./data/{}_history_prices.csv", symb.to_string());
//                 if let Ok(recs) = Intraday::price_records(&curs[0]) {
//                     utils::writerecs(prices_fn, &["date_time", &curs[0].ticker.to_string()], recs);
//                 }
//             } else {
//                 println!("currency route missing: {}", symb.to_string());
//                 continue;
//             }
//         }
//     }
//     Ok(())
// }

// pub fn commodities_prices(start: String) -> Result<(), reqwest::Error> {
//     let index = COMMODITIES_SYMBOLS
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
//     for s in todo_symbs.iter() {
//         if let Some(hist) = get_history(format!("{}%3ACOM", s.to_string())) {
//             if let Ok(prices) = Intraday::price_records(&hist[0]) {
//                 let prices_fn = format!("./data/{}_prices.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 utils::writerecs(prices_fn, &["date_time", &price_col], prices);
//             }
//             if let Ok(volume) = Intraday::volume_records(&hist[0]) {
//                 let volume_fn = format!("./data/{}_volume.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 utils::writerecs(volume_fn, &["date_time", &vol_col], volume);
//             }
//         }
//     }
//     Ok(())
// }

// pub fn commodities_intraday() -> Result<(), reqwest::Error> {
//     // let index = COMMODITIES_SYMBOLS
//     //     .iter()
//     //     .position(|r| r.to_string() == start.to_string())
//     //     .unwrap();

//     // let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
//     for s in COMMODITIES_SYMBOLS.iter() {
//         if let Some(hist) = get_history(format!("{}%3ACOM", s.to_string())) {
//             if let Ok(prices) = Intraday::price_records(&hist[0]) {
//                 let prices_fn = format!("./data/{}_intraday_prices.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 utils::writerecs(prices_fn, &["date_time", &price_col], prices);
//             }
//             if let Ok(volume) = Intraday::volume_records(&hist[0]) {
//                 let volume_fn = format!("./data/{}_intraday_volume.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 utils::writerecs(volume_fn, &["date_time", &vol_col], volume);
//             }
//         }
//     }
//     Ok(())
// }

pub const PRICE_HEADER: [&'static str; 2] = ["date_time", "price"];
pub const NEWS_HEADER: [&'static str; 3] = ["url", "headline", "date_time"];
pub const HEADLINES_HEADER: [&'static str; 4] = ["id", "url", "headline", "lastmod"];

pub fn news() -> Result<(), csv::Error> {
    let write_fn = "./ref_data/news.csv";
    let mut wtr = csv::Writer::from_path(&write_fn)?;
    wtr.write_record(&NEWS_HEADER);
    for s in NEWS_SYMBOLS.iter() {
        if let Some(news_vec) = get_news(s.to_string()) {
            if let Ok(recs) = news::NewsVec::to_records(&news_vec) {
                for r in recs.iter() {
                    wtr.write_record(r);
                }
            }
        }
    }
    wtr.flush();

    Ok(())
}

pub fn sp500(start: String, write_header: bool) -> Result<(), csv::Error> {
    let symbs = utils::read_tickers("./ref_data/sp500tickers.txt");
    let index = symbs
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &symbs[index..symbs.len()];

    let headlines_fn = "./ref_data/sp500_headlines.csv".to_string();
    let metadata_fn = "./ref_data/sp500.csv".to_string();
    let mut meta_wtr = csv::Writer::from_path(&metadata_fn)?;
    let mut lines_wtr = csv::Writer::from_path(&headlines_fn)?;
    meta_wtr.write_record(&STOCK_HEADER);
    lines_wtr.write_record(&HEADLINES_HEADER);
    for s in todo_symbs.iter() {
        let symb = format!("{}%3AUS", s.to_string());
        if let Some(c) = get_datastrip(symb.to_string()) {
            if let Ok(headlines) = Root::to_headlines(&c[0]) {
                for r in headlines.iter() {
                    lines_wtr.write_record(r);
                }
            }
            let metadata_record = Root::to_record(&c[0]);
            meta_wtr.write_record(&metadata_record);
        }
    }
    meta_wtr.flush();
    lines_wtr.flush();
    Ok(())
}

// pub fn stock_prices(start: String) -> Result<(), reqwest::Error> {
//     let symbs = utils::read_tickers("./data/sp500tickers.txt");
//     let index = symbs
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &symbs[index..symbs.len()];
//     for s in todo_symbs.iter() {
//         if let Some(hist) = get_history(format!("{}%3AUS", s.to_string())) {
//             if let Ok(recs) = Intraday::price_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_history_price.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 utils::writerecs(write_fn, &["date_time", &price_col], recs);
//             }
//             if let Ok(recs) = Intraday::volume_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_history_vol.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 utils::writerecs(write_fn, &["date_time", &vol_col], recs);
//             }
//         }
//     }
//     Ok(())
// }
// pub fn stock_intraday(start: String) -> Result<(), reqwest::Error> {
//     let symbs = utils::read_tickers("./data/sp500tickers.txt");
//     let index = symbs
//         .iter()
//         .position(|r| r.to_string() == start.to_string())
//         .unwrap();

//     let todo_symbs = &symbs[index..symbs.len()];
//     for s in todo_symbs.iter() {
//         let symb = format!("{}%3AUS", s.to_string());
//         if let Some(hist) = get_intraday_or_history(symb.to_string()) {
//             if let Ok(recs) = Intraday::price_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_intraday_price.csv", s.to_string());
//                 let price_col = format!("{}_price", &s.to_string());
//                 utils::writerecs(write_fn, &["date_time", &price_col], recs);
//             }
//             if let Ok(recs) = Intraday::volume_records(&hist[0]) {
//                 let write_fn = format!("./data/{}_stock_intraday_vol.csv", s.to_string());
//                 let vol_col = format!("{}_volume", &s.to_string());
//                 utils::writerecs(write_fn, &["date_time", &vol_col], recs);
//             }
//         }
//     }
//     Ok(())
// }

pub fn currency_urls() -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    for s1 in CURRENCY_SYMBOLS.iter() {
        for s2 in CURRENCY_SYMBOLS.iter() {
            if s1 == s2 {
                continue;
            }
            let symb = format!("{}{}:CUR", s1.to_string(), s2.to_string());

            urls.push(bloomberg_url(utils::Security::X(symb)));
        }
    }
    return urls;
}

pub fn us_tickers() -> Vec<String> {
    let urls = utils::read_tickers("./ref_data/tickers.txt")
        .iter()
        .map(|x| bloomberg_url(utils::Security::US(format!("{}:US", x))))
        .collect();
    println!("{:#?}", urls);
    return urls;
}

pub fn bloomberg_url(s: utils::Security) -> String {
    let root = "https://www.bloomberg.com/";

    let intra_prefix = "markets2/api/intraday/";
    let intra_sfx = "?days=10&interval=0&volumeInterval=0";
    // https://www.bloomberg.com/markets2/api/intraday/USDJPY%3ACUR?days=10&interval=0&volumeInterval=0
    // let hist_prefix = "markets2/api/history/";
    // "&limit=1000"
    // let news_prefix"/markets/api/comparison/news?securityType="
    // let news_sfx "/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily"

    match s {
        utils::Security::F(s) => vec![root, intra_prefix, &s, intra_sfx].join(""),
        utils::Security::X(s) => vec![root, intra_prefix, &s, intra_sfx].join(""),
        utils::Security::US(s) => vec![root, intra_prefix, &s, intra_sfx].join(""),
    }
}

pub fn get_datastrip(url: String) -> Option<Vec<Root>> {
    if let Ok(body) = getters::simple_get(url) {
        let company: Vec<Root> = serde_json::from_str(&body.to_string()).unwrap();
        if company != vec![] {
            return Some(company);
        }
    }
    None
}

pub fn get_intraday_or_history(url: String) -> Option<Vec<Intraday>> {
    if let Ok(body) = getters::simple_get(url) {
        let cur: Vec<Intraday> = serde_json::from_str(&body.to_string()).unwrap();
        if cur != vec![] {
            return Some(cur);
        }
    }
    None
}

pub fn get_news(url: String) -> Option<news::NewsVec> {
    if let Ok(body) = getters::simple_get(url) {
        let cur: news::NewsVec = serde_json::from_str(&body.to_string()).unwrap();
        if cur.news != vec![] {
            return Some(cur);
        }
    }
    None
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub average_days_to_maturity: ::serde_json::Value,
    pub average_volume30_day: ::serde_json::Value,
    pub back_load_fee: ::serde_json::Value,
    pub bbid: String,
    pub bics_industry: String,
    pub bics_sector: String,
    pub bics_sub_industry: String,
    pub co_fund_manager: ::serde_json::Value,
    pub company_address: String,
    pub company_description: String,
    pub company_is_private: bool,
    pub company_phone: ::serde_json::Value,
    pub company_website: ::serde_json::Value,
    pub current_management_fee: ::serde_json::Value,
    pub dividend: ::serde_json::Value,
    pub earnings_announcement: ::serde_json::Value,
    pub earnings_per_share: ::serde_json::Value,
    pub price_earnings_to_growth_and_dividend_yield_ratio: ::serde_json::Value,
    pub expense_ratio: ::serde_json::Value,
    pub founded_year: ::serde_json::Value,
    pub front_load_fee: ::serde_json::Value,
    pub fundamental_data_currency: String,
    pub fund_asset_class_focus: ::serde_json::Value,
    pub fund_geographic_focus: ::serde_json::Value,
    pub fund_manager: ::serde_json::Value,
    pub fund_marketing_fee: ::serde_json::Value,
    pub fund_objective: ::serde_json::Value,
    pub fund_type: ::serde_json::Value,
    pub gics_industry: i64,
    pub gics_sector: i64,
    pub high_price: f64,
    pub high_price52_week: f64,
    pub id: String,
    pub inception_date: ::serde_json::Value,
    pub index_description: ::serde_json::Value,
    pub index_source: ::serde_json::Value,
    pub indicated_gross_dividend_yield: ::serde_json::Value,
    pub is_open: bool,
    pub issued_currency: String,
    pub last_announcement_period: String,
    pub last_dividend_reported: ::serde_json::Value,
    pub last_update: String,
    pub long_name: String,
    pub low_price: f64,
    pub low_price52_week: f64,
    pub market_cap: f64,
    pub market_status: String,
    pub media_security_type: String,
    pub media_security_subtype: String,
    pub name: String,
    pub net_asset_value: ::serde_json::Value,
    pub net_asset_value_date: ::serde_json::Value,
    pub next_earnings_announcement: ::serde_json::Value,
    pub next_earnings_period: ::serde_json::Value,
    pub next_earnings_period_end: ::serde_json::Value,
    pub number_of_employees: ::serde_json::Value,
    pub open_price: f64,
    pub parent_ticker: String,
    pub percent_premium: ::serde_json::Value,
    pub percent_premium52_week_average: ::serde_json::Value,
    pub percent_change1_day: f64,
    pub periodicity: ::serde_json::Value,
    pub previous_closing_price_one_trading_day_ago: f64,
    pub price: f64,
    pub price_change1_day: f64,
    pub price_earnings_ratio: ::serde_json::Value,
    pub price_min_decimals: i64,
    pub price_to_book_ratio: ::serde_json::Value,
    pub price_to_sales_ratio: ::serde_json::Value,
    pub primary_exchange: String,
    pub redemption_fee: ::serde_json::Value,
    pub score: ::serde_json::Value,
    pub security_name: ::serde_json::Value,
    pub share_class: ::serde_json::Value,
    pub shares_outstanding: i64,
    pub short_name: String,
    pub time_zone_offset: i64,
    pub total_assets: ::serde_json::Value,
    pub total_assets_date: ::serde_json::Value,
    pub total_assets_currency: ::serde_json::Value,
    pub total_return1_year: ::serde_json::Value,
    pub total_return3_month: ::serde_json::Value,
    pub total_return3_year: ::serde_json::Value,
    pub total_return5_year: ::serde_json::Value,
    pub total_return_ytd: ::serde_json::Value,
    pub trading_day_close: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ultimate_parent_ticker: String,
    pub volume: i64,
    pub press_releases: Option<Vec<PressRelease>>,
}

impl Root {
    pub fn to_record(&self) -> csv::StringRecord {
        let rec = vec![
            self.id.to_string(),
            self.short_name.to_string(),
            self.market_cap.to_string(),
            self.company_phone.to_string(),
            self.last_update.to_string(),
            self.average_volume30_day.to_string(),
            self.price.to_string(),
            self.open_price.to_string(),
            self.high_price.to_string(),
            self.low_price.to_string(),
            self.low_price52_week.to_string(),
            self.high_price52_week.to_string(),
            self.number_of_employees.to_string(),
            self.price_earnings_ratio.to_string(),
            self.shares_outstanding.to_string(),
        ];
        return csv::StringRecord::from(rec);
    }

    pub fn to_headlines(&self) -> Result<Vec<csv::StringRecord>, &'static str> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        if let Some(prs) = &self.press_releases {
            for pr in prs.iter() {
                ret.push(PressRelease::to_record(pr));
            }
            Ok(ret)
        } else {
            Err("no headlines most likely")
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressRelease {
    pub id: String,
    pub url: String,
    pub headline: Headline,
    pub updated_at: String,
}

impl PressRelease {
    pub fn to_record(&self) -> csv::StringRecord {
        let hl_text = self.headline.text.replace(",", ";");
        let rec = &[
            self.id.to_string(),
            self.url.to_string(),
            hl_text.to_string(),
            self.updated_at.to_string(),
        ];
        return csv::StringRecord::from(rec.to_vec());
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intraday {
    // #[serde(flatten)]
    pub ticker: String,
    pub previous_closing_price_one_trading_day_ago: ::serde_json::Value,
    pub open_price: ::serde_json::Value,
    pub range: Option<Range>,
    pub price: Vec<Price>,
    pub volume: Vec<Volume>,
}

impl Intraday {
    pub fn price_records(&self) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        // for i in 0..self.price.len() {
        //     let rec = vec!(
        //         self.price[i].date_time.to_string(),
        //         self.price[i].value.to_string(),
        //     );
        for p in self.price.iter() {
            ret.push(Price::to_record(p));
        }
        return ret;
    }

    pub fn volume_records(&self) -> Result<Vec<csv::StringRecord>, &'static str> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        for i in 0..self.volume.len() {
            let rec = [
                self.volume[i].date_time.to_string(),
                self.volume[i].value.to_string(),
            ];
            ret.push(csv::StringRecord::from(rec.to_vec()));
        }
        Ok(ret)
    }

    // pub fn write_records(&self, fn:String) -> Result<(), &'static str> {
    //     let recs = self.to_records();
    //     let header: [&'static str; 2] = ["date_time", &self.ticker.to_string()];
    //     utils::utils::writerecs(fn, header, recs);
    //     Ok(())
    // }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub date_time: String,
    pub value: f64,
}

impl Price {
    pub fn to_record(&self) -> Vec<String> {
        //csv::StringRecord::from
        return vec![self.date_time.to_string(), self.value.to_string()];
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub date_time: String,
    pub value: i64,
}
