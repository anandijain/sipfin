extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate csv;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResMeta {
    pub more: bool,
    pub minimum_input_length: i64,
    pub class_code: String,
    pub class_name: String,
    pub results: Vec<MetaResult>,
}

impl ResMeta {
    pub fn to_records(&self) -> Vec<csv::StringRecord> {
        let mut recs: Vec<csv::StringRecord> = Vec::new();
        for elt in self.results.iter(){
            recs.push(MetaResult::to_record(elt));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaResult {
    pub id: String,
    pub text: String,
    pub parent: String,
}

impl MetaResult {
    pub fn to_record(&self) -> csv::StringRecord {
        let text = self.text.replace(",", ";");
        let rec = &[
                self.id.to_string(),
                text.to_string(),
                self.parent.to_string(),
            ];
        return csv::StringRecord::from(rec.to_vec());
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Areas {
    pub more: bool,
    pub results: Vec<AreaResult>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaResult {
    pub id: String,
    pub text: String,
}
