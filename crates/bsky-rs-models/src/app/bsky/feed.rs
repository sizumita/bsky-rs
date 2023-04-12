use crate::com::atproto::repo::StrongRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionView {
    pub id: i32,
    pub action: ActionType,
    pub subject: ActionViewSubject,
    #[serde(rename = "subjectBlobCids")]
    pub subject_blob_cids: Vec<String>,
    pub reason: String,
    #[serde(rename = "createdBy")]
    pub created_by: String, // TODO: format: did
    #[serde(rename = "createdAt")]
    pub created_at: String, // TODO: format: datetime
    pub reversal: ActionReversal,
    #[serde(rename = "resolvedReportIds")]
    pub resolved_report_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// TODO enum serialize type for json
pub enum ActionViewSubject {
    RepoRef(RepoRef),
    StrongRef(StrongRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionType {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoRef {
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionReversal {
    // TODO
}
