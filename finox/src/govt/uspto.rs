// https://events.api.uspto.gov/v1/events
// https://developer.uspto.gov/ibd-api/v1/patent/application
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsptoApplications {
    pub response: EventResponse,
}

impl crate::HasRecs for UsptoApplications {
    fn to_recs(&self) -> Vec<Vec<String>> {
        self.response.docs.iter().map(|x| x.to_rec()).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventResponse {
    pub num_found: i64,
    pub start: i64,
    pub docs: Vec<Doc>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub application_type: String,
    pub document_id: String,
    pub application_number: String,
    pub document_type: String,
    pub patent_number: String,
    pub publication_date: String,
    pub document_date: String,
    pub production_date: String,
    pub application_date: String,
    pub applicant: Vec<String>,
    pub inventor: Vec<String>,
    #[serde(default)]
    pub assignee: Vec<String>,
    pub title: String,
    pub archive_url: String,
    pub pdf_path: String,
    pub year: String,
    #[serde(rename = "_version_")]
    pub version: i64,
}

impl Doc {
    pub fn to_rec(&self) -> Vec<String> {
        vec![
            self.document_id.to_string(),
            self.application_type.to_string(),
            self.application_number.to_string(),
            self.document_type.to_string(),
            self.patent_number.to_string(),
            self.publication_date.to_string(),
            self.document_date.to_string(),
            self.production_date.to_string(),
            self.application_date.to_string(),
            self.applicant.join(" ").to_string(),
            self.inventor.join(" ").to_string(),
            self.assignee.join(" ").to_string(),
            self.title.to_string(),
            self.archive_url.to_string(),
            self.pdf_path.to_string(),
            self.year.to_string(),
            self.version.to_string(),
        ]
    }
}

pub const PATENT_HEADER: [&'static str; 17] = [
    "document_id",
    "application_type",
    "application_number",
    "document_type",
    "patent_number",
    "publication_date",
    "document_date",
    "production_date",
    "application_date",
    "applicant",
    "inventor",
    "assignee",
    "title",
    "archive_url",
    "pdf_path",
    "year",
    "version",
];

//https://developer.uspto.gov/ptab-api/decisions
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decisions {
    pub aggregation_data: ::serde_json::Value,
    pub results: Vec<Decision>,
    pub record_total_quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decision {
    pub proceeding_number: String,
    pub decision_type_category: String,
    pub subdecision_type_category: String,
    pub document_name: String,
    pub proceeding_type_category: String,
    pub subproceeding_type_category: String,
    pub document_identifier: String,
    pub respondent_technology_center_number: String,
    pub respondent_patent_owner_name: Option<String>,
    pub respondent_party_name: String,
    pub respondent_group_art_unit_number: String,
    pub respondent_counsel_name: Option<String>,
    pub respondent_patent_number: Option<String>,
    pub respondent_application_number_text: String,
    pub petitioner_party_name: Option<String>,
    pub petitioner_counsel_name: Option<String>,
    pub ocr_search_text: String,
    pub issue_type: Vec<String>,
    pub board_rulings: Vec<String>,
    pub decision_date: String,
    pub respondent_grant_date: Option<String>,
    pub identifier: String,
    pub additional_respondent_party_data: Vec<::serde_json::Value>,
    pub appellant_technology_center_number: Option<String>,
    pub appellant_patent_owner_name: Option<String>,
    pub appellant_party_name: Option<String>,
    pub appellant_group_art_unit_number: Option<String>,
    pub appellant_inventor_name: Option<String>,
    pub appellant_counsel_name: Option<String>,
    pub appellant_application_number_text: Option<String>,
    pub appellant_publication_date: Option<String>,
    pub appellant_publication_number: Option<String>,
}
