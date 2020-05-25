extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// https://www.jpx.co.jp/english/news/news_ym_01.json

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub kind: String,
    pub category: Vec<String>,
    pub corporation: Vec<String>,
    #[serde(rename = "ir_category")]
    pub ir_category: Vec<String>,
    #[serde(rename = "product_category")]
    pub product_category: Vec<String>,
    pub title: String,
    pub url: String,
    #[serde(rename = "updated_date")]
    pub updated_date: JPXUpdatedDate,
    #[serde(rename = "display_type")]
    pub display_type: String,
    #[serde(rename = "external_flg")]
    pub external_flg: Vec<String>,
    #[serde(rename = "extension_icon")]
    pub extension_icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JPXUpdatedDate {
    pub year: String,
    pub month: String,
    pub day: String,
}

impl Root {
    pub fn to_record(&self) -> Vec<String> {
        let ret: Vec<String> = vec![
            self.kind.to_string(),
            self.category[0].to_string(),
            self.corporation[0].to_string(),
            self.ir_category[0].to_string(),
            self.product_category[0].to_string(),
            self.title.replace(",", " "),
            self.url.to_string(),
            self.updated_date.year.to_string(),
            self.updated_date.month.to_string(),
            self.updated_date.day.to_string(),
        ];
        return ret;
    }
}

pub const JPXNewsHeader: [&'static str; 10] = [
    "kind",
    "category",
    "corporation",
    "ir_category",
    "product_category",
    "title",
    "url",
    "year",
    "month",
    "day",
];
