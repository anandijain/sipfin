// https://video-api.wsj.com/api-video/find_all_videos.asp
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJRoot {
    pub items: Vec<WSJVideos>,
}

impl crate::HasRecs for WSJRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for hl in self.items.iter() {
            recs.push(WSJVideos::to_rec(hl));
        }
        return recs;
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
