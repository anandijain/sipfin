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
    let ns: Vec<String> = (1..10).map(|x| (x * 100).to_string()).collect();

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
    let fetches = finox::fetch_into::<uspto::UsptoApplications>(hm.values().cloned().collect())
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    println!("{:#?}", fetches);
    Ok(())
}
