extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResMeta {
    pub more: bool,
    pub minimum_input_length: i64,
    pub class_code: String,
    pub class_name: String,
    pub results: Vec<MetaResult>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaResult {
    pub id: String,
    pub text: String,
    pub parent: String,
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
