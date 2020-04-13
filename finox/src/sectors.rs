#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub sector_tree: SectorTree,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorTree {
    pub security: String,
    pub id: String,
    pub name: String,
    pub ni_codes: Vec<String>,
    pub children: Vec<Children>,
    pub percent_change1_day: f64,
    pub weight: i64,
    pub last_update_epoch: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub security: String,
    pub id: i64,
    pub name: String,
    pub ni_codes: Vec<String>,
    pub children: Vec<Children2>,
    pub percent_change1_day: f64,
    pub weight: f64,
    pub last_update_epoch: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    pub security: String,
    pub id: i64,
    pub name: String,
    pub ni_codes: Vec<String>,
    pub percent_change1_day: f64,
    pub weight: f64,
    pub last_update_epoch: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub id: String,
    pub articles: Vec<Article>,
    pub videos: Vec<Video>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub headline: String,
    pub url: String,
    pub summary: String,
    pub updated_at: String,
    pub thumbnail: Thumbnail,
    #[serde(rename = "updatedAtISO")]
    pub updated_at_iso: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub base_url: String,
    pub orig_width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub headline: String,
    pub url: String,
    pub summary: String,
    pub updated_at: String,
    pub thumbnail: Thumbnail2,
    #[serde(rename = "updatedAtISO")]
    pub updated_at_iso: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail2 {
    pub base_url: String,
}
