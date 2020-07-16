// https://video-api.wsj.com/api-video/find_all_videos.asp
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJRoot {
    pub items: Vec<WSJVideos>,
}

impl crate::HasRecs for WSJRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.items.iter().map(|x| x.to_rec()).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJVideos {
    pub id: String,
    pub unix_creation_date: i64,
    pub name: String,
    pub description: String,
    pub duration: String,
    #[serde(rename = "thumbnailURL")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "videoURL")]
    pub video_url: Option<String>,
    #[serde(rename = "emailURL")]
    pub email_url: Option<String>,
    #[serde(rename = "doctypeID")]
    pub doctype_id: Option<String>,
    pub column: Option<String>,
}

impl WSJVideos {
    pub fn to_rec(&self) -> Vec<String> {
        return vec![
            self.id.to_string(),
            self.unix_creation_date.to_string(),
            self.name.to_string(),
            self.description.to_string(),
            self.duration.to_string(),
            self.column.clone().unwrap_or("".to_string()),
            self.doctype_id.clone().unwrap_or("".to_string()),
            self.email_url.clone().unwrap_or("".to_string()),
            self.thumbnail_url.clone().unwrap_or("".to_string()),
        ];
    }
}

// https://www.wsj.com/news/archive/2003/12/22?id=%7B%22params%22%3A%20%7B%20%22timeout%22%3A%20%222000%22%2C%20%22query%22%3A%22%22%2C%22count%22%3A%20%22200%22%2C%22max-date%22%3A%20%222003%2F12%2F22%22%2C%22min-date%22%3A%20%222003%2F12%2F22%22%7D%2C%22clientId%22%3A%20%22grandcanyon%22%2C%22database%22%3A%20%22wsjie%22%7D&type=dnsasearch_full

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJArchive {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: ::serde_json::Value,
    pub collection: Vec<HeadlineIDs>,
    pub hash: ::serde_json::Value,
}

impl crate::HasRecs for WSJArchive {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.collection
            .iter()
            .map(|x| x.to_rec())
            .collect::<Vec<_>>()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadlineIDs {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl HeadlineIDs {
    pub fn to_rec(&self) -> Vec<String> {
        vec![self.id.to_string(), self.type_field.to_string()]
    }
}

// https://www.wsj.com/news/archive/2003/12/22?id=SB107214762693982900&type=article%7Cdnsa
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJArticleRoot {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: ::serde_json::Value,
    pub hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJArticle {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub article_id: String,
    pub article_section: String,
    pub article_type: String,
    pub flashline: ::serde_json::Value,
    pub headline: String,
    pub byline: Option<String>,
    pub image: ::serde_json::Value,
    pub pub_date: ::serde_json::Value,
    pub title: String,
    pub summary: String,
    pub summaries: ::serde_json::Value,
    pub comment_count: Option<i64>,
    pub timestamp: i64,
    pub url: String,
    pub video: bool,
    pub entitlements: Vec<::serde_json::Value>,
}
