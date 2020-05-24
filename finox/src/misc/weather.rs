extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::fs;
use crate::utils;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: i64,
    pub name: String,
    pub state: String,
    pub country: String,
    pub coord: Coord,
}
impl Root {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec!(
            self.id.to_string(),
            self.name.to_string(),
            self.state.to_string(),
            self.country.to_string(),
        );
        rec.append(&mut Coord::to_record(&self.coord));

        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

impl Coord {
    pub fn to_record(&self) -> Vec<String> {
        return vec![self.lat.to_string(), self.lon.to_string()];
    }
}


pub fn cities_to_csv() -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path("city_list.csv".to_string())?;
    let jsonstr = fs::read_to_string("city_list.json")
        .expect("Something went wrong reading the file");
    let roots: Vec<Root> = serde_json::from_str(&jsonstr.to_string()).unwrap();
    let recs: Vec<csv::StringRecord> = roots
            .into_iter()
            .map(|x| csv::StringRecord::from(Root::to_record(&x)))
            .collect();
    wtr.write_record(vec!(
        "id",
        "name",
        "state",
        "country",
        "lat",
        "lon",
    ));
    for r in recs.iter() {
        wtr.write_record(r);
    }
    wtr.flush()?;
    println!("With text:\n{:#?}", recs);
    Ok(())
}
