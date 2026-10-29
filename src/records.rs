//! Schema v1 record output. Account and plugin ownership come from the host.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecordResult {
    pub schema_version: String,
    pub sources: Vec<SourceRef>,
    pub records: Vec<RecordContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refresh_requests: Vec<RefreshRequest>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    pub id: String,
    pub kind: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct RecordIdentity {
    pub namespace: String,
    pub key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldCandidate {
    pub value: Value,
    pub evidence: Vec<String>,
    pub basis: String,
    pub confidence: f64,
    pub observed_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SemanticUpdate {
    pub key: String,
    pub occurred_at: String,
    pub summary: String,
    pub notify: bool,
    pub evidence: Vec<String>,
    pub fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordContribution {
    pub identity: RecordIdentity,
    #[serde(rename = "type")]
    pub record_type: String,
    pub fields: BTreeMap<String, FieldCandidate>,
    pub updates: Vec<SemanticUpdate>,
    pub attention: Vec<Value>,
    pub links: Vec<Value>,
    pub presentation: Value,
    pub lifecycle: Value,
}

/// Host-mediated GET request. The host owns approval, credentials and scheduling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RefreshRequest {
    pub identity: RecordIdentity,
    pub service: String,
    pub path: String,
    pub interval_seconds: u64,
}

/// Input to the optional `refresh(ptr, len)` export, using the process packing ABI.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshInput {
    pub schema_version: String,
    pub invocation: RefreshInvocation,
    pub service: ServiceResponse,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshInvocation {
    pub schema_version: String,
    pub kind: String,
    pub capabilities: Vec<String>,
    pub config: Value,
    pub records: Vec<Value>,
    pub sources: Vec<SourceRef>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceResponse {
    /// Host fetch time, independent of provider response contents.
    #[serde(default)]
    pub fetched_at: Option<String>,
    pub id: String,
    pub resource: String,
    pub source_id: String,
    pub response: Value,
}
