// https://www.moodys.com/_layouts/mdc/am/Request/request.php?profile=homepage

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: i64,
    pub message: String,
    pub polltime: i64,
    pub timestamp: String,
    pub count: i64,
    pub headlines: Vec<Headline>,
}

impl crate::HasRecs for Root {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.headlines.iter().map(|x| x.to_rec()).collect()
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub backfill: bool,
    pub is_web_feed: bool,
    pub title: String,
    #[serde(rename = "read_key")]
    pub read_key: String,
    pub source: String,
    #[serde(rename = "receive_date")]
    pub receive_date: String,
    pub synopsis: String,
    pub url: String,
    pub symbols: Vec<String>,
    pub codes: Vec<String>,
    
}

impl Headline {
    pub fn to_rec(&self) -> Vec<String> {
        vec![
            self.title.to_string(),
            self.source.to_string(),
            self.receive_date.to_string(),
            self.synopsis.to_string(),
        ]
    }
}
