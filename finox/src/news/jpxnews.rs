extern crate serde;
extern crate serde_derive;
extern crate serde_json;

// https://www.jpx.co.jp/english/news/news_ym_01.json

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Root {
    pub kind: String,
    pub category: Vec<String>,
    pub corporation: Vec<String>,
    pub ir_category: Vec<String>,
    pub product_category: Vec<String>,
    pub title: String,
    pub url: String,
    pub updated_date: JPXUpdatedDate,
    pub display_type: String,
    pub external_flg: Vec<String>,
    pub extension_icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JPXUpdatedDate {
    pub year: String,
    pub month: String,
    pub day: String,
}

impl Root {
    pub fn to_rec(&self) -> Vec<String> {
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

