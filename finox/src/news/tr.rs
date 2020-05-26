// https://sope.prod.reuters.tv/program/rcom/v1/article-recirc?edition=cn&modules=rightrail,ribbon,bottom

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TRRoot {
    pub rightrail: TRRibbon,
    pub ribbon: TRRibbon,
    pub bottom: TRRibbon,
}

impl crate::HasRecs for TRRoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for list in [&self.rightrail, &self.ribbon, &self.bottom].iter() {
            recs.append(&mut TRRibbon::to_recs(list));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TRRibbon {
    #[serde(rename = "ab_test")]
    pub ab_test: Vec<::serde_json::Value>,
    pub errors: Vec<::serde_json::Value>,
    pub stories: Vec<TRStory>,
    pub tags: Vec<String>,
}

impl TRRibbon{
    pub fn to_recs(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for s in self.stories.iter() {
            recs.push(TRStory::to_rec(&s));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TRStory {
    pub updated: i64,
    pub headline: String,
    pub image: String,
    pub reason: String,
    pub path: String,
    pub id: String,
    pub channel: ::serde_json::Value,
}

impl TRStory {
    pub fn to_rec(&self) -> Vec<String> {
       return vec![ 
            self.id.to_string(),
            self.updated.to_string(),
            self.headline.replace(",", ";").to_string(),
            self.reason.to_string(),
            self.path.to_string(),
        ];
    }
}
