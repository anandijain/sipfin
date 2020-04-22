extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate csv;

pub const CURRENCY_SYMBOLS: [&'static str; 40] = [
    "USD", "EUR", "XAU", "XAG", "XPT", "XPD", "JPY", "GBP", "AUD", "CAD", "CHF", "KRW", "MXN",
    "BRL", "CLP", "COP", "PEN", "CRC", "ARS", "SEK", "DKK", "NOK", "CZK", "SKK", "PLN", "HUF",
    "RUB", "TRY", "ILS", "KES", "ZAR", "MAD", "NZD", "PHP", "SGD", "IDR", "CNY", "INR", "MYR",
    "THB",
];

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




pub fn currencies_intraday(start: String) -> Result<(), reqwest::Error> {
    let index = CURRENCY_SYMBOLS
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();
    let todo_symbs = &CURRENCY_SYMBOLS[index..CURRENCY_SYMBOLS.len()];
    for s1 in todo_symbs.iter() {
        for s2 in CURRENCY_SYMBOLS.iter() {
            if s1 == s2 {
                continue;
            }
            let symb = format!("{}{}%3ACUR", s1.to_string(), s2.to_string());
            if let Some(curs) = getters::get_intraday(symb.to_string()) {
                let prices_fn = format!("./data/{}_intraday_prices.csv", symb.to_string());
                if let Ok(recs) = types::Intraday::price_records(&curs[0]) {
                    writerecs(prices_fn, &["date_time", &curs[0].ticker.to_string()], recs);
                }
            } else {
                println!("currency route missing: {}", symb.to_string());
                continue;
            }
        }
    }
    Ok(())
}

pub fn currencies_history(start: String) -> Result<(), reqwest::Error> {
    let index = CURRENCY_SYMBOLS
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &CURRENCY_SYMBOLS[index..CURRENCY_SYMBOLS.len()];
    for s1 in todo_symbs.iter() {
        for s2 in CURRENCY_SYMBOLS.iter() {
            if s1 == s2 {
                continue;
            }
            let symb = format!("{}{}%3ACUR", s1.to_string(), s2.to_string());
            if let Some(curs) = getters::get_history(symb.to_string()) {
                let prices_fn = format!("./data/{}_history_prices.csv", symb.to_string());
                if let Ok(recs) = types::Intraday::price_records(&curs[0]) {
                    writerecs(prices_fn, &["date_time", &curs[0].ticker.to_string()], recs);
                }
            } else {
                println!("currency route missing: {}", symb.to_string());
                continue;
            }
        }
    }
    Ok(())
}

pub fn commodities_prices(start: String) -> Result<(), reqwest::Error> {
    let index = COMMODITIES_SYMBOLS
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
    for s in todo_symbs.iter() {
        if let Some(hist) = getters::get_history(format!("{}%3ACOM", s.to_string())) {
            if let Ok(prices) = types::Intraday::price_records(&hist[0]) {
                let prices_fn = format!("./data/{}_prices.csv", s.to_string());
                let price_col = format!("{}_price", &s.to_string());
                writerecs(prices_fn, &["date_time", &price_col], prices);
            }
            if let Ok(volume) = types::Intraday::volume_records(&hist[0]) {
                let volume_fn = format!("./data/{}_volume.csv", s.to_string());
                let vol_col = format!("{}_volume", &s.to_string());
                writerecs(volume_fn, &["date_time", &vol_col], volume);
            }
        }
    }
    Ok(())
}

pub fn commodities_intraday() -> Result<(), reqwest::Error> {
    // let index = COMMODITIES_SYMBOLS
    //     .iter()
    //     .position(|r| r.to_string() == start.to_string())
    //     .unwrap();

    // let todo_symbs = &COMMODITIES_SYMBOLS[index..COMMODITIES_SYMBOLS.len()];
    for s in COMMODITIES_SYMBOLS.iter() {
        if let Some(hist) = getters::get_history(format!("{}%3ACOM", s.to_string())) {
            if let Ok(prices) = types::Intraday::price_records(&hist[0]) {
                let prices_fn = format!("./data/{}_intraday_prices.csv", s.to_string());
                let price_col = format!("{}_price", &s.to_string());
                writerecs(prices_fn, &["date_time", &price_col], prices);
            }
            if let Ok(volume) = types::Intraday::volume_records(&hist[0]) {
                let volume_fn = format!("./data/{}_intraday_volume.csv", s.to_string());
                let vol_col = format!("{}_volume", &s.to_string());
                writerecs(volume_fn, &["date_time", &vol_col], volume);
            }
        }
    }
    Ok(())
}

pub fn news() -> Result<(), csv::Error> {
    let write_fn = "./data/news.csv";
    let mut wtr = csv::Writer::from_path(&write_fn)?;
    wtr.write_record(&NEWS_HEADER);
    for s in NEWS_SYMBOLS.iter() {
        if let Some(news_vec) = getters::get_news(s.to_string()) {
            if let Ok(recs) = news::NewsVec::to_records(&news_vec) {
                for r in recs.iter() {
                    wtr.write_record(r);
                }
            }
        }
    }
    Ok(())
}

pub fn sp500(start: String, write_header: bool) -> Result<(), csv::Error> {
    let symbs = read_tickers("./data/sp500tickers.txt");
    let index = symbs
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &symbs[index..symbs.len()];

    let headlines_fn = "./data/sp500_headlines.csv".to_string();
    let metadata_fn = "./data/sp500.csv".to_string();
    let mut meta_wtr = csv::Writer::from_path(&metadata_fn)?;
    let mut lines_wtr = csv::Writer::from_path(&headlines_fn)?;
    meta_wtr.write_record(&STOCK_HEADER);
    lines_wtr.write_record(&HEADLINES_HEADER);
    for s in todo_symbs.iter() {
        let symb = format!("{}%3AUS", s.to_string());
        if let Some(c) = getters::get_datastrip(symb.to_string()) {
            if let Ok(headlines) = types::Root::to_headlines(&c[0]) {
                for r in headlines.iter() {
                    lines_wtr.write_record(r);
                }
            }
            let metadata_record = types::Root::to_record(&c[0]);
            meta_wtr.write_record(&metadata_record);
        }
    }
    meta_wtr.flush();
    lines_wtr.flush();
    Ok(())
}

pub fn stock_prices(start: String) -> Result<(), reqwest::Error> {
    let symbs = read_tickers("./data/sp500tickers.txt");
    let index = symbs
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &symbs[index..symbs.len()];
    for s in todo_symbs.iter() {
        if let Some(hist) = getters::get_history(format!("{}%3AUS", s.to_string())) {
            if let Ok(recs) = types::Intraday::price_records(&hist[0]) {
                let write_fn = format!("./data/{}_stock_history_price.csv", s.to_string());
                let price_col = format!("{}_price", &s.to_string());
                writerecs(write_fn, &["date_time", &price_col], recs);
            }
            if let Ok(recs) = types::Intraday::volume_records(&hist[0]) {
                let write_fn = format!("./data/{}_stock_history_vol.csv", s.to_string());
                let vol_col = format!("{}_volume", &s.to_string());
                writerecs(write_fn, &["date_time", &vol_col], recs);
            }
        }
    }
    Ok(())
}
pub fn stock_intraday(start: String) -> Result<(), reqwest::Error> {
    let symbs = read_tickers("./data/sp500tickers.txt");
    let index = symbs
        .iter()
        .position(|r| r.to_string() == start.to_string())
        .unwrap();

    let todo_symbs = &symbs[index..symbs.len()];
    for s in todo_symbs.iter() {
        let symb = format!("{}%3AUS", s.to_string());
        if let Some(hist) = getters::get_intraday(symb.to_string()) {
            if let Ok(recs) = types::Intraday::price_records(&hist[0]) {
                let write_fn = format!("./data/{}_stock_intraday_price.csv", s.to_string());
                let price_col = format!("{}_price", &s.to_string());
                writerecs(write_fn, &["date_time", &price_col], recs);
            }
            if let Ok(recs) = types::Intraday::volume_records(&hist[0]) {
                let write_fn = format!("./data/{}_stock_intraday_vol.csv", s.to_string());
                let vol_col = format!("{}_volume", &s.to_string());
                writerecs(write_fn, &["date_time", &vol_col], recs);
            }
        }
    }
    Ok(())
}

match 
US(s) => vec![s, "%3AUS"].join(""),
F(s) => vec![s, "%3ACOM"].join("")

pub fn bloomberg_url(s: Security) -> String {
    let root = "https://www.bloomberg.com/";
    let prefix = "markets2/api/datastrip/";

    let intra_prefix = "/markets2/api/intraday/";
    let intra_sfx = "?days=10&interval=0&volumeInterval=0";
    
    let hist_prefix = "markets2/api/history/";
    "&limit=1000"
    let news_prefix"/markets/api/comparison/news?securityType="
    let news_sfx "/PX_LAST?timeframe=5_YEAR&period=daily&volumePeriod=daily"

    match s {
        Security::F(s) => vec![root, &s, "=F?symbol=", &s, sfx].join(""),
        Security::X(s) => vec![root, &s, "=X?symbol=", &s, sfx].join(""),
        Security::US(s) => vec![root, &s, "?region=US", sfx].join(""),
    }
}

pub fn get_datastrip(t: String) -> Option<Vec<types::Root>> {
    let url = [
        
        &t,
        "%3AUS",
    
    if let Ok(body) = simple_get(url) {
        let company: Vec<types::Root> = serde_json::from_str(&body.to_string()).unwrap();
        if company != vec![] {
            return Some(company);
        }
    }
    None
}

pub fn get_intraday(t: String) -> Option<Vec<types::Intraday>> {
    if let Ok(body) = simple_get(url) {
        let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
        if cur != vec![] {
            return Some(cur);
        }
    }
    None
}

pub fn get_history(t: String) -> Option<Vec<types::Intraday>> {
    if let Ok(body) = simple_get(url) {
        let cur: Vec<types::Intraday> = serde_json::from_str(&body.to_string()).unwrap();
        if cur != vec![] {
            return Some(cur);
        }
    }
    None
}

pub fn get_news(t: String) -> Option<news::NewsVec> {

    if let Ok(body) = simple_get(url) {
        let cur: news::NewsVec = serde_json::from_str(&body.to_string()).unwrap();
        if cur.news != vec![] {
            return Some(cur);
        }
    }
    None
}