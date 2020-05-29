use serde_json::Value;
use std::{collections::HashMap, time::Duration};

pub const GUARDIAN_DELAY: Duration = Duration::from_millis(100);

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionsRoot {
    pub response: SectionsResponse,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionsResponse {
    pub status: String,
    pub user_tier: String,
    pub total: i64,
    pub results: Vec<SectionsResult>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionsResult {
    pub id: String,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleRoot {
    pub response: ArticleResponse,
}

impl crate::HasRecs for ArticleRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.response.results.iter().map(|x| x.to_rec()).collect()
    }
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub results: Vec<ArticleResult>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResult {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub section_id: String,
    pub section_name: String,
    pub web_publication_date: String,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    pub is_hosted: bool,
    pub pillar_id: Option<String>,
    pub pillar_name: Option<String>,
}

impl ArticleResult {
    pub fn to_rec(&self) -> Vec<String> {
        return vec![
            self.id.to_string(),
            self.type_field.to_string(),
            self.section_id.to_string(),
            self.section_name.to_string(),
            self.web_publication_date.to_string(),
            self.web_title.to_string(),
            self.api_url.to_string(),
            self.is_hosted.to_string(),
            self.pillar_id.clone().unwrap_or("".to_string()),
        ];
    }
}
