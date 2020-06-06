use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SecIndex {
    pub cik: u64,
    pub company_name: String,
    pub form_type: String,
    pub date_filed: String,
    pub filename: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Sec13F {
    pub name_of_issuer: String,
    pub title_of_class: String,
    pub cusip: String,
    pub value: String,
    pub ssh_pr_amt: String,
    pub ssh_pr_amt_type: String,
    pub investment_discretion: String,
    //pub other_manager: String,
    pub sole_voting: String,
    pub shared_voting: String,
    pub no_voting: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SecFormHeader {
    pub cik: String,
    pub date: String,
    pub company_name: String,
    pub company_address: String,
    pub company_city: String,
    pub company_state_country: String,
    pub zip_code: String,
    pub signee: String,
    pub signee_title: String,
    pub signee_phone: String,
    pub signature: String,
    pub signee_city: String,
    pub signee_state_country: String,
    pub sign_date: String,
    pub other_managers_count: String,
    pub row_count: String,
    pub total_value: String,
    pub is_confidential_omitted: String,
}

pub fn sec_header(input: &str) -> Option<SecFormHeader> {
    //regex::re_unicode::Captures{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<cik>(?P<cik>.*)</cik>(.|\n)*?<periodOfReport>(?P<date>.*)</periodOfReport>(.|\n)*?<filingManager>\s*<name>(?P<company_name>.*)</name>\s*<address>\s*<com:street1>(?P<street_address>.*)</com:street1>(.|\n)*?<com:city>(?P<city>.*)</com:city>\s*<com:stateOrCountry>(?P<stateOrCountry>.*)</com:stateOrCountry>\s*<com:zipCode>(?P<zipCode>.*)</com:zipCode>\s*</address>(.|\n)*?<signatureBlock>\s*<name>(?P<signee>.*)</name>\s*<title>(?P<signee_title>.*)</title>\s*<phone>(?P<phone>.*)</phone>\s*<signature>(?P<signature>.*)</signature>\s*<city>(?P<signee_city>.*)</city>\s*<stateOrCountry>(?P<signee_state>.*)</stateOrCountry>\s*<signatureDate>(?P<sign_date>.*)</signatureDate>\s*</signatureBlock>\s*<summaryPage>\s*<otherIncludedManagersCount>(?P<other_managers_count>.*)</otherIncludedManagersCount>\s*<tableEntryTotal>(?P<row_count>.*)</tableEntryTotal>\s*<tableValueTotal>(?P<value_total>.*)</tableValueTotal>\s*<isConfidentialOmitted>(?P<is_confidential>.*)</isConfidentialOmitted>\s*</summaryPage>")
        .unwrap();
    }
    for cap in RE.captures_iter(input) {
        return Some(SecFormHeader {
            cik: cap["cik"].to_string(),
            date: cap["date"].to_string(),
            company_name: cap["company_name"].to_string(),
            company_address: cap["street_address"].to_string(),
            company_city: cap["city"].to_string(),
            company_state_country: cap["stateOrCountry"].to_string(),
            zip_code: cap["zipCode"].to_string(),
            signee: cap["signee"].to_string(),
            signee_title: cap["signee_title"].to_string(),
            signee_phone: cap["phone"].to_string(),
            signature: cap["signature"].to_string(),
            signee_city: cap["signee_city"].to_string(),
            signee_state_country: cap["signee_state"].to_string(),
            sign_date: cap["sign_date"].to_string(),
            other_managers_count: cap["other_managers_count"].to_string(),
            row_count: cap["row_count"].to_string(),
            total_value: cap["value_total"].to_string(),
            is_confidential_omitted: cap["is_confidential"].to_string(),
        });
    }
    None
}

pub fn sec_13f(input: &str) -> Option<Vec<Sec13F>> {
    // TODO, misses put rows
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"<infoTable>\s*<nameOfIssuer>(?P<name>.+)</nameOfIssuer>\s*<titleOfClass>(?P<title>.+)</titleOfClass>\s*<cusip>(?P<cusip>.+)</cusip>\s*<value>(?P<value>.+)</value>\s*<shrsOrPrnAmt>\s*<sshPrnamt>(?P<amt>.+)</sshPrnamt>\s*<sshPrnamtType>(?P<amt_type>.+)</sshPrnamtType>\s*</shrsOrPrnAmt>\s*<investmentDiscretion>(?P<discretion>.+)</investmentDiscretion>(.|\n)*?<votingAuthority>\s*<Sole>(?P<sole>.+)</Sole>\s*<Shared>(?P<shared>.+)</Shared>\s*<None>(?P<none>.+)</None>\s*</votingAuthority>\s*</infoTable>"
        )
        .unwrap();
    }
    let mut recs = vec![];
    for cap in RE.captures_iter(input) {
        recs.push(Sec13F {
            name_of_issuer: cap["name"].to_string(),
            title_of_class: cap["title"].to_string(),
            cusip: cap["cusip"].to_string(),
            value: cap["value"].to_string(),
            ssh_pr_amt: cap["amt"].to_string(),
            ssh_pr_amt_type: cap["amt_type"].to_string(),
            investment_discretion: cap["discretion"].to_string(),
            //other_manager: cap["other_manager"].to_string(),
            sole_voting: cap["sole"].to_string(),
            shared_voting: cap["shared"].to_string(),
            no_voting: cap["none"].to_string(),
        })
    }
    if recs.len() > 0 {
        return Some(recs);
    }
    None
}
