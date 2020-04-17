extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

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


// https://sope.prod.reuters.tv/program/rcom/v1/article-recirc?edition=cn&modules=rightrail,ribbon,bottom

// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TR {
//     pub rightrail: TRRibbon,
//     pub ribbon: TRRibbon,
//     pub bottom: TRRibbon,
// }

// impl TR {
//     pub fn to_records(&self) -> Vec<Vec<String>> {
//         let mut recs: Vec<Vec<String>> = Vec::new();
//         for hl in self.list.iter(){
//             recs.push(Headline::to_record(hl));
//         }
//         return recs;
//     }
// }


// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TRRibbon {
//     #[serde(rename = "ab_test")]
//     pub ab_test: Vec<::serde_json::Value>,
//     pub errors: Vec<::serde_json::Value>,
//     pub stories: Vec<Story>,
//     pub tags: Vec<String>,
// }
// impl TRRibbon {
//     pub fn to_record(&self) -> Vec<Vec<String>> {
//         let rec: Vec<String> = vec!(
//             self.id.to_string(),

//             );
//         return rec;
//     }
// }

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    pub updated: i64,
    pub headline: String,
    pub image: String,
    pub reason: String,
    pub path: String,
    pub id: String,
    pub channel: Channel,
}

impl Story {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.id.to_string(),
            self.updated.to_string(),
            self.headline.to_string(),
            self.reason.to_string(),
            self.path.to_string(),
            self.channel.name.to_string(),
            );
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub path: String,
    pub name: String,
}


// https://video-api.wsj.com/api-video/find_all_videos.asp

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJ {
    pub items: Vec<WSJArticle>,
}
impl WSJ {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for hl in self.items.iter(){
            recs.push(WSJArticle::to_record(hl));
        }
        return recs;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSJArticle {
    pub id: String,
    pub unix_creation_date: i64,
    pub name: String,
    pub description: String,
    pub duration: String,
    #[serde(rename = "thumbnailURL")]
    pub thumbnail_url: String,
    #[serde(rename = "videoURL")]
    pub video_url: String,
    #[serde(rename = "emailURL")]
    pub email_url: String,
    #[serde(rename = "doctypeID")]
    pub doctype_id: String,
    pub column: String,
}

impl WSJArticle {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.id.to_string(),
            self.unix_creation_date.to_string(),
            self.name.to_string(),
            self.description.to_string(),
            self.duration.to_string(),
            self.column.to_string(),
            self.doctype_id.to_string(),
            self.email_url.to_string(),
            );
        return rec;
    }
}

// https://api.nytimes.com/svc/news/v3/content/all/all.json
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTFeed {
    pub status: String,
    pub copyright: Option<String>,
    #[serde(rename = "num_results")]
    pub num_results: i64,
    pub results: Vec<NYTFeedArticle>,
}

impl NYTFeed {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for article in self.results.iter(){
            recs.push(NYTFeedArticle::to_record(article));
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTFeedArticle {
    #[serde(rename = "slug_name")]
    pub slug_name: String,
    pub section: String,
    pub subsection: String,
    pub title: String,
    #[serde(rename = "abstract")]
    pub abstract_field: String,
    pub url: String,
    pub byline: String,
    #[serde(rename = "thumbnail_standard")]
    pub thumbnail_standard: Option<String>,
    #[serde(rename = "item_type")]
    pub item_type: String,
    pub source: String,
    #[serde(rename = "updated_date")]
    pub updated_date: String,
    #[serde(rename = "created_date")]
    pub created_date: String,
    #[serde(rename = "published_date")]
    pub published_date: String,
    #[serde(rename = "first_published_date")]
    pub first_published_date: String,
    #[serde(rename = "material_type_facet")]
    pub material_type_facet: String,
    pub kicker: String,
    pub subheadline: String,
    #[serde(rename = "des_facet")]
    #[serde(default)]
    pub des_facet: Option<Vec<String>>,
    #[serde(rename = "org_facet")]
    #[serde(default)]
    pub org_facet: Option<Vec<String>>,
    #[serde(rename = "per_facet")]
    #[serde(default)]
    pub per_facet: Option<Vec<String>>,
    #[serde(rename = "geo_facet")]
    #[serde(default)]
    pub geo_facet: Option<Vec<String>>,
    #[serde(rename = "related_urls")]
    pub related_urls: ::serde_json::Value,
    pub uri: String,
    pub multimedia: Option<Vec<NYTFeedMultimedia>>,
}

impl NYTFeedArticle {
    pub fn to_record(&self) -> Vec<String> {
        
        let rec: Vec<String> = vec!(
            self.slug_name.to_string(),
            self.section.to_string(),
            self.subsection.to_string(),
            self.title.replace(",", ";").to_string(),
            self.abstract_field.replace(",", ";").to_string(),
            self.byline.to_string(),
            self.item_type.to_string(),
            self.source.to_string(),
            self.first_published_date.to_string(),
            self.created_date.to_string(),
            self.published_date.to_string(),
            self.updated_date.to_string(),
            self.material_type_facet.to_string(),
            self.url.to_string(),
            );
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTFeedMultimedia {
    pub url: Option<String>,
    pub format: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub subtype: Option<String>,
    pub caption: Option<String>,
    pub copyright: Option<String>,
}

// https://api.nytimes.com/svc/archive/v1/1926/1.json
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTArchive {
    pub copyright: Option<String>,
    pub response: NYTArchiveResponse,
}

impl NYTArchive {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for article in self.response.docs.iter(){
            recs.push(NYTArchiveArticle::to_record(article));
        }
        return recs;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTArchiveResponse {
    pub meta: NYTArchiveMeta,
    pub docs: Vec<NYTArchiveArticle>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTArchiveMeta {
    pub hits: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTArchiveArticle {
    #[serde(rename = "web_url")]
    pub web_url: String,
    pub snippet: Option<String>,
    #[serde(rename = "lead_paragraph")]
    pub lead_paragraph: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_field: Option<String>,
    #[serde(rename = "print_page")]
    pub print_page: Option<String>,
    pub blog: Option<Vec<::serde_json::Value>>,
    pub source: String,
    pub multimedia: Vec<::serde_json::Value>,
    pub headline: NYTArchiveHeadline,
    pub keywords: Vec<Keyword>,
    #[serde(rename = "pub_date")]
    pub pub_date: String,
    #[serde(rename = "document_type")]
    pub document_type: String,
    #[serde(rename = "news_desk")]
    pub news_desk: Option<serde_json::Value>,
    #[serde(rename = "section_name")]
    pub section_name: Option<serde_json::Value>,
    #[serde(rename = "subsection_name")]
    pub subsection_name: Option<serde_json::Value>,
    pub byline: Option<Byline>,
    #[serde(rename = "type_of_material")]
    pub type_of_material: Option<String>,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "word_count")]
    pub word_count: i64,
    #[serde(rename = "slideshow_credits")]
    pub slideshow_credits: Option<serde_json::Value>,
}

impl NYTArchiveArticle {
    pub fn to_record(&self) -> Vec<String> {
        // let first_name = lilmatcher(self.byline.person.firstname);
        // let first_name = lilmatcher(self.byline.person.middlename);
        // let first_name = lilmatcher(self.byline.person.lastname);
        
        let orig: String = byline_orig(self.byline.clone());
        let snip = lilmatcher(self.snippet.clone());
        let abs_field = lilmatcher(self.abstract_field.clone());
        let page = lilmatcher(self.print_page.clone());
        let kicker = lilmatcher(self.headline.kicker.clone());
        
        let rec: Vec<String> = vec!(
            self.id.to_string(),
            self.word_count.to_string(),
            orig.replace(",", ";").to_string(),
            self.pub_date.to_string(),
            self.document_type.to_string(),
            page.to_string(),
            self.headline.main.replace(",", ";").to_string(),
            kicker.replace(",", ";").to_string(),
            snip.replace(",", ";").to_string(),
            abs_field.replace(",", ";").to_string(),
            self.web_url.to_string(),
            self.source.to_string(),
            );
        return rec;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NYTArchiveHeadline {
    pub main: String,
    pub kicker: Option<String>,
    #[serde(rename = "content_kicker")]
    pub content_kicker: Option<String>,
    #[serde(rename = "print_headline")]
    pub print_headline: Option<String>,
    pub name: Option<serde_json::Value>,
    pub seo:  Option<serde_json::Value>,
    pub sub:  Option<serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    pub name: Option<String>,
    pub value: Option<String>,
    pub rank: Option<i64>,
    pub major: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Byline {
    pub original: Option<String>,
    #[serde(default)]
    pub person: Option<Vec<Person>>,
    pub organization: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub firstname: Option<String>,
    pub middlename: Option<String>,
    pub lastname: Option<String>,
    pub qualifier: Option<String>,
    pub title: Option<serde_json::Value>,
    pub role: String,
    pub organization: String,
    pub rank: i64,
}

pub fn byline_orig(byline: Option<Byline>) -> String {
    if let Some(byline) = byline {
        return lilmatcher(byline.original);
    }
    return "".to_string();
}

pub fn lilmatcher(s: Option<String>) -> String {
    match s{
            Some(s) => s.to_string(),
            None => "".to_string(),
    }
}
