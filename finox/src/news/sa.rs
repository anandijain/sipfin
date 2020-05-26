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
pub struct SARoot {
    pub list: Vec<List>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct List {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub slug: Option<String>,
    pub company_name: Option<String>,
    pub author_picture: String,
    pub author_name: ::serde_json::Value,
    pub publish_on: i64,
    pub comments_counts: String,
    pub author_user_id: i64,
}

impl crate::HasRecs for SARoot {
    fn to_recs(&self) -> Vec<Vec<String>> {
        return self.list.iter().map(|x| x.to_rec()).collect::<Vec<_>>(); 
    }
}

impl List{
    pub fn to_rec(&self) -> Vec<String> {
        let slug = match self.slug.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let rec: Vec<String> = vec![
            self.id.to_string(),
            self.author_user_id.to_string(),
            self.publish_on.to_string(),
            self.title.replace(",", ";").to_string(),
            slug.to_string(),
            self.comments_counts.to_string(),
            self.author_name.to_string().replace(",", ";"),
            self.path.to_string(),
        ];
        return rec;
    }
}
