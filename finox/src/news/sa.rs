extern crate serde;
extern crate serde_derive;
extern crate serde_json;

/*
https://finance.api.seekingalpha.com/v2/real-time-prices?symbols%5B%5D=GOOG
https://seekingalpha.com/tooltips/get
https://seekingalpha.com/news/trending_news
https://seekingalpha.com/symbol/AAPL/financials-data?period_type=quarter&statement_type=income-statement&is_pro=true
https://seekingalpha.com/account/ajax_get_comments?id=4337864&type=Article
*/


// https://seekingalpha.com/get_trending_articles
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub list: Vec<Headline>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub slug: Option<String>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "author_picture")]
    pub author_picture: String,
    #[serde(rename = "author_name")]
    pub author_name: ::serde_json::Value,
    #[serde(rename = "publish_on")]
    pub publish_on: i64,
    #[serde(rename = "comments_counts")]
    pub comments_counts: String,
    #[serde(rename = "author_user_id")]
    pub author_user_id: i64,
}

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for hl in self.list.iter(){
            recs.push(Headline::to_record(hl));
        }
        return recs;
    }
}

impl Headline {
    pub fn to_record(&self) -> Vec<String> {
        let slug = match self.slug.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let rec: Vec<String> = vec!(
            self.id.to_string(),
            self.author_user_id.to_string(),
            self.publish_on.to_string(),
            self.title.replace(",", ";").to_string(),
            slug.to_string(),
            self.comments_counts.to_string(),
            self.author_name.to_string().replace(",", ";"),
            self.path.to_string(),
            );
        return rec;
    }
}