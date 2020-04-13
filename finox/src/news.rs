extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsVec {
    pub news: Vec<News>,
}

impl NewsVec {
    pub fn to_records(&self) -> Result<Vec<csv::StringRecord>, csv::Error> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        for article in self.news.iter() {
                ret.push(News::to_record(article));
            }
        
        Ok(ret)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub headline: String,
    pub published_at: String,
    pub url: String,
    #[serde(rename = "publishedAtISO")]
    pub published_at_iso: String,
}

impl News {
    pub fn to_record(&self) -> csv::StringRecord {
        let hl_text = self.headline.replace(",", ";");
        let rec = &[
            self.url.to_string(),
            hl_text.to_string(),
            self.published_at.to_string(),
        ];
        return csv::StringRecord::from(rec.to_vec());
    }
}