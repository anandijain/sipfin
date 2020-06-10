//https://api.gsa.gov/technology/digital-registry/v1/social_media
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GovtSocials {
    pub metadata: Metadata,
    pub results: Vec<Result>,
}

impl crate::HasRecs for GovtSocials {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.results.iter().map(|x| x.to_rec()).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Metadata {
    pub count: i64,
    pub page: i64,
    pub page_size: i64,
    pub pages: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Result {
    pub id: i64,
    pub organization: String,
    pub account: Option<String>,
    pub service_key: String,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub service_display_name: String,
    pub service_url: String,
    pub language: String,
    pub agencies: Vec<Agency>,
    pub tags: Vec<Tag>,
    pub created_at: String,
    pub updated_at: String,
}

impl Result {
    pub fn to_rec(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.organization.to_string(),
            //self.account.to_string(),
            self.service_key.to_string(),
            self.service_url.to_string(),
            self.short_description
                .clone()
                .unwrap_or("".into())
                .to_string(),
            self.long_description
                .clone()
                .unwrap_or("".into())
                .to_string(),
            self.language.to_string(),
            self.created_at.to_string(),
            self.updated_at.to_string(),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Agency {
    pub id: i64,
    pub name: String,
    pub info_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tag {
    pub id: i64,
    pub tag_text: String,
}

pub const GOVT_SOCIALS_HEADER: [&'static str; 9] = [
    "id",
    "org",
    "service",
    "url",
    "short_desc",
    "long_desc",
    "lang",
    "created",
    "updated",
];
