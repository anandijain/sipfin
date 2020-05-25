extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use finox;
//use finox::news;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // utils::nytarchive();

    //finox::news::nytfeed();
    finox::gsnews()?;
    //utils::jpxnews();
    //finox::reuters();
    //finox::wsj_videos();
    finox::sa()?;
    //bloomberg::news();
    Ok(())
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsVec {
    pub news: Vec<News>,
}

impl NewsVec {
    pub fn to_records(&self) -> Result<Vec<csv::StringRecord>, csv::Error> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        for article in self.news.iter() {
            ret.push(News::to_record(article));
        }
        Ok(ret)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub headline: String,
    pub published_at: String,
    pub url: String,
    #[serde(rename = "publishedAtISO")]
    pub published_at_iso: String,
}

impl News {
    pub fn to_record(&self) -> csv::StringRecord {
        let hl_text = self.headline.replace(",", ";");
        let rec = &[
            self.url.to_string(),
            hl_text.to_string(),
            self.published_at.to_string(),
        ];
        return csv::StringRecord::from(rec.to_vec());
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub path: String,
    pub name: String,
}

// https://sope.prod.reuters.tv/program/rcom/v1/article-recirc?edition=cn&modules=rightrail,ribbon,bottom

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TR {
    pub rightrail: TRRibbon,
    pub ribbon: TRRibbon,
    pub bottom: TRRibbon,
}

impl TR {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for list in [&self.rightrail, &self.ribbon, &self.bottom].iter() {
            recs.append(&mut TRRibbon::to_records(list));
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

impl TRRibbon {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for s in self.stories.iter() {
            recs.push(TRStory::to_record(&s));
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
    pub channel: Channel,
}

impl TRStory {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec![
            self.id.to_string(),
            self.updated.to_string(),
            self.headline.replace(",", ";").to_string(),
            self.reason.to_string(),
            self.path.to_string(),
            self.channel.name.to_string(),
            self.channel.path.to_string(),
        ];
        return rec;
    }
}


