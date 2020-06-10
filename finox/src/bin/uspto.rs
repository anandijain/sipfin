use csv;
use finox::{
    govt::{
        socials::{GovtSocials, GOVT_SOCIALS_HEADER},
        uspto,
    },
    HasRecs,
};

use std::{collections::HashMap, env, fs};
#[tokio::main]
pub async fn main() -> Result<(), csv::Error> {
    let ns: Vec<String> = (1..1000).map(|x| (x * 100).to_string()).collect();

    let mut hm: HashMap<String, String> = HashMap::new();
    for elt in ns.iter() {
        hm.insert(
            elt.into(),
            format!(
                "https://developer.uspto.gov/ibd-api/v1/patent/application?start={}&rows=100",
                elt
            ),
        );
    }

    println!("{:#?}", hm);
    finox::fetch_write::<uspto::UsptoApplications>(
        hm,
        "../data/uspto/applications/",
        &uspto::PATENT_HEADER,
    )
    .await
    .expect("idk how long thisll take");

    Ok(())
}

pub fn json_to_csv() -> Result<(), csv::Error> {
    let args = env::args().collect::<Vec<String>>();
    let output = args[1].to_string();
    let base = output.split(".").collect::<Vec<_>>()[0];

    let f = fs::read_to_string(output.clone()).expect("couldnt read file to string");
    let data = serde_json::from_str::<GovtSocials>(&f)
        .expect("couldn't into json")
        .to_recs();
    println!("{:#?}", data);
    let mut wtr = csv::Writer::from_path(format!("{}.csv", base))?;
    wtr.write_record(GOVT_SOCIALS_HEADER.to_vec())?;

    for d in data.iter() {
        wtr.write_record(d)?;
    }
    wtr.flush()?;
    Ok(())
}
