extern crate chrono;
extern crate csv;
use chrono::Utc;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread,
    time::Duration,
};

pub const DELAY: std::time::Duration = Duration::from_millis(10);

pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36";

#[tokio::main]
pub async fn simple_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.to_string())
        .build()?;
    let res = client.get(&url).send().await?;
    thread::sleep(DELAY);
    let body = res.text().await?;
    // println!("{}: {:#?}", url, body);
    println!("{}", url);
    Ok(body)
}

#[tokio::main]
pub async fn simple_json(url: String) -> Result<::serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.to_string())
        .build()?;

    client
        .get(&url)
        .send()
        .await?
        .json::<::serde_json::Value>() // CHANGE TYPE
        .await
}

pub fn writerecs(
    file_name: String,
    header: &[&str],
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name).expect("csv messed up");
    wtr.write_record(header)?;
    for r in records.iter() {
        wtr.write_record(r)?;
    }
    Ok(())
}

pub fn write_csv(
    filepath: &Path,
    data: Vec<Vec<String>>,
    header: &[&str],
) -> Result<(), csv::Error> {
    let mut wtr =
        csv::Writer::from_path(filepath).expect(format!("whtf csv {:?}", filepath).as_ref());
    wtr.write_record(header.clone())?;
    wtr.flush()?;
    let len = header.len();
    for row in data.iter() {
        assert_eq!(len, row.len()); // perf hit?
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}


pub fn appendrecs(
    file_name: String,
    records: Vec<csv::StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let file = std::fs::OpenOptions::new().append(true).open(file_name)?;
    let mut wtr = csv::Writer::from_writer(file);
    for r in records.iter() {
        wtr.write_record(r)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn simppath(s: String, sfx: String) -> String {
    //sfx enum x, f, us
    return format!(
        "../data/{}_{}_{}.csv",
        s.to_string(),
        sfx.to_string(),
        Utc::now().to_rfc3339(),
    );
}

pub fn regexmain() -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open("rentec_13f.xml")?;
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    let res = vec![
        Regex::new(r"<nameOfIssuer>(?P<val>.+)</nameOfIssuer>.*()").unwrap(),
        Regex::new(r"<titleOfClass>(?P<val>.+)</titleOfClass>.*()").unwrap(),
        Regex::new(r"<cusip>(?P<val>.+)</cusip>.*()").unwrap(),
        Regex::new(r"<value>(?P<val>.+)</value>.*()").unwrap(),
        Regex::new(r"<sshPrnamt>(?P<val>.+)</sshPrnamt>.*()").unwrap(),
        Regex::new(r"<sshPrnamtType>(?P<val>.+)</sshPrnamtType>.*()").unwrap(),
        Regex::new(r"<investmentDiscretion>(?P<val>.+)</investmentDiscretion>.*()").unwrap(),
        Regex::new(r"<otherManager>(?P<val>.+)</otherManager>.*()").unwrap(),
        Regex::new(r"<Sole>(?P<val>.+)</Sole>.*()").unwrap(),
        Regex::new(r"<Shared>(?P<val>.+)</Shared>.*()").unwrap(),
        Regex::new(r"<None>(?P<val>.+)</None>.*()").unwrap(),
    ];
    // buf_reader.read_to_string(&mut contents)?;
    let filenames = read_tickers("../ref_data/rentec13urls.txt");
    for (i, url) in filenames.iter().enumerate() {
        let mut allcaps: Vec<Vec<String>> = Vec::new();
        let contents = simple_get(url.to_string()).unwrap();
        for re in res.iter() {
            let mut rec: Vec<String> = Vec::new();
            for cap in re.captures_iter(&contents.to_string()) {
                if let Some(val) = cap.name("val") {
                    rec.push(val.as_str().to_string());
                } else {
                    println!("OH FUCK");
                    rec.push("".to_string());
                }
            }
            allcaps.push(rec);
        }
        let path = format!(
            "./ref_data/rentec/regex_rentec_holdings_{}.csv",
            i.to_string()
        );
        let mut wtr = csv::Writer::from_path(path)?;
        let len = allcaps[0].len();
        for vec in allcaps.iter() {
            assert_eq!(len, vec.len());
            let rec = csv::StringRecord::from(vec.clone());
            wtr.write_record(&rec)?;
        }
        wtr.flush()?;
    }
    Ok(())
}

pub fn yf_symb_from_url(url: String) -> Option<String> {
    //example
    let re = Regex::new(r"/chart/(?P<symb>.+).*\?").unwrap();
    if let Some(caps) = re.captures(&url) {
        return Some(caps.name("symb").unwrap().as_str().to_string());
    }
    return None;
}

pub fn symb_from_ndaq_url(url: String) -> Option<String> {
    //example
    let re = Regex::new(r"/quote/(?P<symb>.+).*/info").unwrap();
    if let Some(caps) = re.captures(&url) {
        return Some(caps.name("symb").unwrap().as_str().to_string());
    }
    return None;
}

