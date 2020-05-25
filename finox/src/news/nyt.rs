
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
        for article in self.results.iter() {
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
    pub multimedia: Option<Vec<NYTFeedMultimedia>>,
    #[serde(rename = "thumbnail_standard")]
    pub thumbnail_standard: Option<String>,
}

impl NYTFeedArticle {
    pub fn to_record(&self) -> Vec<String> {
        //limiting 1 for tags
        let geo = match &self.geo_facet {
            Some(s) => s[0].replace(",", ";").to_string(),
            None => "".to_string(),
        };
        let org = match &self.org_facet {
            Some(s) => s[0].replace(",", ";").to_string(),
            None => "".to_string(),
        };
        let des = match &self.des_facet {
            Some(s) => s[0].replace(",", ";").to_string(),
            None => "".to_string(),
        };
        let per = match &self.per_facet {
            Some(s) => s[0].replace(",", ";").to_string(),
            None => "".to_string(),
        };

        let thumbnail_url = utils::lilmatcher(self.thumbnail_standard.clone());

        let rec: Vec<String> = vec![
            self.slug_name.to_string(),
            self.first_published_date.to_string(),
            self.section.to_string(),
            self.subsection.to_string(),
            self.byline.replace(",", ";").to_string(),
            self.title.replace(",", ";").to_string(),
            self.subheadline.replace(",", ";").to_string(),
            self.abstract_field.replace(",", ";").to_string(),
            self.material_type_facet.to_string(),
            geo.to_string(),
            org.to_string(),
            des.to_string(),
            per.to_string(),
            self.source.to_string(),
            self.published_date.to_string(),
            self.created_date.to_string(),
            self.updated_date.to_string(),
            self.url.to_string(),
            thumbnail_url.to_string(),
            self.kicker.to_string(),
            self.item_type.to_string(),
        ];
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
        for article in self.response.docs.iter() {
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
        let snip = utils::lilmatcher(self.snippet.clone());
        let abs_field = utils::lilmatcher(self.abstract_field.clone());
        let page = utils::lilmatcher(self.print_page.clone());
        let kicker = utils::lilmatcher(self.headline.kicker.clone());

        let rec: Vec<String> = vec![
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
        ];
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
    pub seo: Option<serde_json::Value>,
    pub sub: Option<serde_json::Value>,
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
        return utils::lilmatcher(byline.original);
    }
    return "".to_string();
}


/*
admin
arts
automobiles
books
briefing
business
climate
corrections
crosswords \u0026 games
education
en español
fashion
food
guides
health
home \u0026 garden
home page
job market
lens
magazine
movies
multimedia/photos
new york
obituaries
opinion
parenting
podcasts
reader center
real estate
science
smarter living
sports
style
sunday review
t brand
t magazine
technology
the learning network
the upshot
the weekly
theater
times insider
today’s paper
travel
u.s.
universal
video
well
world
your money

*/
