extern crate serde;
extern crate serde_derive;
extern crate serde_json;


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graphics {
    pub items: Vec<Graphic>,
    pub expanded: ::serde_json::Value,
    pub cached_module: ::serde_json::Value,
    pub name: String, // asserteq!(name, "Graphics")
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graphic {
    pub attachments: Attachments,
    pub credits: Credits,
    pub headline: String,
    pub headlines: Headlines,
    pub id: String,
    pub metadata: Metadata,
    pub extra_media: ExtraMedia,
    pub minor_updated_at: String,
    pub published_at: String,
    pub quote: ::serde_json::Value,
    pub revision: String,
    pub slug: String,
    pub summary: String,
    pub content_tags: Vec<ContentTag>,
    pub tags: Tags,
    #[serde(rename = "type")]
    pub type_field: String,
    pub updated_at: String,
    pub url: String,
    pub primary_site: String,
    pub related_stories: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachments {
    pub image: ::serde_json::Value,
    pub video: Video,
    pub video_audio: VideoAudio,
}

//uhoh
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Img {
    pub base_url: String,
    pub description: String,
    pub orig_width: i64,
    pub title: String,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credits {
    pub author: Vec<Author>,
    pub by: Vec<Author>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub slug: String,
    pub full_name: String,
    pub image: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headlines {
    pub web: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub apple_news_free: bool,
    pub disable_ads: bool,
    pub exclude_from_cliff: bool,
    pub exclude_from_paywall: bool,
    pub social: Social,
    pub google_standout: bool,
    pub original: Original,
    pub diff: ::serde_json::Value,
    pub magazine: Option<Magazine>,
    pub theme: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Social {
    pub headline: String,
    pub description: String,
    pub facebook_status: String,
    pub twitter_text: String,
    pub twitter_title: String,
    pub twitter_description: String,
    pub twitter_handle: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Original {
    pub content_tags: Vec<ContentTag>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentTag {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub direct_score: Option<f64>,
    pub derived_score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Magazine {
    pub display_strap: ::serde_json::Value,
    pub headline_short: String,
    pub section: String,
    pub page_number: i64,
    pub short_deck: ::serde_json::Value,
    pub postscript: ::serde_json::Value,
    pub document_version: ::serde_json::Value,
    pub platform_version: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraMedia {
    pub thumbnail: ::serde_json::Value,
    pub social: Social2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Social2 {
    pub default: ::serde_json::Value,
    pub twitter: Option<Twitter>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Twitter {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "_links")]
    pub links: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub editorial_topics: Vec<EditorialTopic>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorialTopic {
    pub id: String,
}
