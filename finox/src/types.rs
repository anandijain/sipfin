extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub average_days_to_maturity: ::serde_json::Value,
    pub average_volume30_day: ::serde_json::Value,
    pub back_load_fee: ::serde_json::Value,
    pub bbid: String,
    pub bics_industry: String,
    pub bics_sector: String,
    pub bics_sub_industry: String,
    pub co_fund_manager: ::serde_json::Value,
    pub company_address: String,
    pub company_description: String,
    pub company_is_private: bool,
    pub company_phone: ::serde_json::Value,
    pub company_website: ::serde_json::Value,
    pub current_management_fee: ::serde_json::Value,
    pub dividend: ::serde_json::Value,
    pub earnings_announcement: ::serde_json::Value,
    pub earnings_per_share: ::serde_json::Value,
    pub price_earnings_to_growth_and_dividend_yield_ratio: ::serde_json::Value,
    pub expense_ratio: ::serde_json::Value,
    pub founded_year: ::serde_json::Value,
    pub front_load_fee: ::serde_json::Value,
    pub fundamental_data_currency: String,
    pub fund_asset_class_focus: ::serde_json::Value,
    pub fund_geographic_focus: ::serde_json::Value,
    pub fund_manager: ::serde_json::Value,
    pub fund_marketing_fee: ::serde_json::Value,
    pub fund_objective: ::serde_json::Value,
    pub fund_type: ::serde_json::Value,
    pub gics_industry: i64,
    pub gics_sector: i64,
    pub high_price: f64,
    pub high_price52_week: f64,
    pub id: String,
    pub inception_date: ::serde_json::Value,
    pub index_description: ::serde_json::Value,
    pub index_source: ::serde_json::Value,
    pub indicated_gross_dividend_yield: ::serde_json::Value,
    pub is_open: bool,
    pub issued_currency: String,
    pub last_announcement_period: String,
    pub last_dividend_reported: ::serde_json::Value,
    pub last_update: String,
    pub long_name: String,
    pub low_price: f64,
    pub low_price52_week: f64,
    pub market_cap: f64,
    pub market_status: String,
    pub media_security_type: String,
    pub media_security_subtype: String,
    pub name: String,
    pub net_asset_value: ::serde_json::Value,
    pub net_asset_value_date: ::serde_json::Value,
    pub next_earnings_announcement: ::serde_json::Value,
    pub next_earnings_period: ::serde_json::Value,
    pub next_earnings_period_end: ::serde_json::Value,
    pub number_of_employees: ::serde_json::Value,
    pub open_price: f64,
    pub parent_ticker: String,
    pub percent_premium: ::serde_json::Value,
    pub percent_premium52_week_average: ::serde_json::Value,
    pub percent_change1_day: f64,
    pub periodicity: ::serde_json::Value,
    pub previous_closing_price_one_trading_day_ago: f64,
    pub price: f64,
    pub price_change1_day: f64,
    pub price_earnings_ratio: ::serde_json::Value,
    pub price_min_decimals: i64,
    pub price_to_book_ratio: ::serde_json::Value,
    pub price_to_sales_ratio: ::serde_json::Value,
    pub primary_exchange: String,
    pub redemption_fee: ::serde_json::Value,
    pub score: ::serde_json::Value,
    pub security_name: ::serde_json::Value,
    pub share_class: ::serde_json::Value,
    pub shares_outstanding: i64,
    pub short_name: String,
    pub time_zone_offset: i64,
    pub total_assets: ::serde_json::Value,
    pub total_assets_date: ::serde_json::Value,
    pub total_assets_currency: ::serde_json::Value,
    pub total_return1_year: ::serde_json::Value,
    pub total_return3_month: ::serde_json::Value,
    pub total_return3_year: ::serde_json::Value,
    pub total_return5_year: ::serde_json::Value,
    pub total_return_ytd: ::serde_json::Value,
    pub trading_day_close: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub ultimate_parent_ticker: String,
    pub volume: i64,
    pub press_releases: Option<Vec<PressRelease>>,
}

impl Root {
    pub fn to_record(&self) -> csv::StringRecord {
        let rec = vec!(
            self.id.to_string(),
            self.short_name.to_string(),
            self.market_cap.to_string(),
            self.company_phone.to_string(),
            self.last_update.to_string(),
            self.average_volume30_day.to_string(),
            self.price.to_string(),
            self.open_price.to_string(),
            self.high_price.to_string(),
            self.low_price.to_string(),
            self.low_price52_week.to_string(),
            self.high_price52_week.to_string(),
            self.number_of_employees.to_string(),
            self.price_earnings_ratio.to_string(),
            self.shares_outstanding.to_string(),
        );
        return csv::StringRecord::from(rec);
    }

    pub fn to_headlines(&self) -> Result<Vec<csv::StringRecord>, &'static str> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        if let Some(prs) = &self.press_releases {
            for pr in prs.iter() {
                ret.push(PressRelease::to_record(pr));
            }
            Ok(ret)
        } else {
            Err("no headlines most likely")
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PressRelease {
    pub id: String,
    pub url: String,
    pub headline: Headline,
    pub updated_at: String,
}

impl PressRelease {
    pub fn to_record(&self) -> csv::StringRecord {
        let hl_text = self.headline.text.replace(",", ";");
        let rec = &[
            self.id.to_string(),
            self.url.to_string(),
            hl_text.to_string(),
            self.updated_at.to_string(),
        ];
        return csv::StringRecord::from(rec.to_vec());
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub text: String,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intraday {
    pub ticker: String,
    pub previous_closing_price_one_trading_day_ago: ::serde_json::Value,
    pub open_price: ::serde_json::Value,
    pub range: Option<Range>,
    pub price: Vec<Price>,
    pub volume: Vec<Volume>,
}

impl Intraday {
    pub fn price_records(&self) -> Result<Vec<csv::StringRecord>, &'static str> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        for i in 0..self.price.len() {
            let rec = [
                self.price[i].date_time.to_string(),
                self.price[i].value.to_string(),
            ];
            ret.push(csv::StringRecord::from(rec.to_vec()));
        }
        Ok(ret)
    }

    pub fn volume_records(&self) -> Result<Vec<csv::StringRecord>, &'static str> {
        let mut ret: Vec<csv::StringRecord> = Vec::new();
        for i in 0..self.volume.len() {
            let rec = [
                self.volume[i].date_time.to_string(),
                self.volume[i].value.to_string(),
            ];
            ret.push(csv::StringRecord::from(rec.to_vec()));
        }
        Ok(ret)
    }

    // pub fn write_records(&self, fn:String) -> Result<(), &'static str> {
    //     let recs = self.to_records();
    //     let header: [&'static str; 2] = ["date_time", &self.ticker.to_string()];
    //     utils::writerecs(fn, header, recs);
    //     Ok(())
    // }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub date_time: String,
    pub value: f64,
}

impl Price {
    pub fn to_record(&self) -> csv::StringRecord {
        return csv::StringRecord::from(vec![self.date_time.to_string(), self.value.to_string()]);
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub date_time: String,
    pub value: i64,
}
