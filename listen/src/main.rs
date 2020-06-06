use csv;
use finox::roses;
use hound;
use std::error::Error;
use std::f32::consts::PI;
use std::i16;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub symbol: String,
    pub t: String,
    pub x: f64,
    pub v: u64,
}

fn main() -> Result<(), hound::Error> {
    //let strs = roses::read_tickers("../ref_data/tickers_stocks.txt");
    //for s in strs.iter() {
    do_one("SPY").unwrap();
    //}
    Ok(())
}

fn do_one(s: &str) -> Result<(), Box<dyn Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let file_name = format!("../data/nasdaq/realtime-trades/{}.csv", s);
    let output_name = format!("../data/sound/{}.wav", s);
    //let mut rdr = csv::Reader::from_path().unwrap();
    //let mut iter = rdr.deserialize();
    let rows = roses::read_into::<Row>(file_name);

    let mut wtr = hound::WavWriter::create(output_name, spec).unwrap();
    for r in rows {
        let sample = r.x * r.v as f64;
        let amplitude = i16::MAX as f32;
        wtr.write_sample((sample * amplitude) as i16).unwrap();
    }
    Ok(())
}
