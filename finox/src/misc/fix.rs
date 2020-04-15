#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Header")]
    pub header: Header,
    #[serde(rename = "Body")]
    pub body: Body,
    #[serde(rename = "Trailer")]
    pub trailer: Trailer,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(rename = "BeginString")]
    pub begin_string: String,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "MsgSeqNum")]
    pub msg_seq_num: String,
    #[serde(rename = "SenderCompID")]
    pub sender_comp_id: String,
    #[serde(rename = "TargetCompID")]
    pub target_comp_id: String,
    #[serde(rename = "SendingTime")]
    pub sending_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(rename = "SecurityIDSource")]
    pub security_idsource: String,
    #[serde(rename = "SecurityID")]
    pub security_id: String,
    #[serde(rename = "MDReqID")]
    pub mdreq_id: String,
    #[serde(rename = "NoMDEntries")]
    pub no_mdentries: Vec<NoMdentry>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoMdentry {
    #[serde(rename = "MDEntryType")]
    pub mdentry_type: String,
    #[serde(rename = "MDEntryPx")]
    pub mdentry_px: String,
    #[serde(rename = "MDEntrySize")]
    pub mdentry_size: String,
    #[serde(rename = "MDEntryTime")]
    pub mdentry_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trailer {
}
