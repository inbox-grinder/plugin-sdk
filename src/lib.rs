pub use serde_json;
pub mod records;
pub use records::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Host-provided configuration snapshot for this invocation.
#[derive(Debug, Clone, Deserialize)]
pub struct InvocationContext {
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub records: Vec<serde_json::Value>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailInput {
    #[serde(default)]
    pub invocation: Option<InvocationContext>,
    pub message: EmailMessageRef,
    pub sender: EmailAddress,
    pub subject: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default, rename = "textBody")]
    pub text_body: Option<String>,
    #[serde(default, rename = "htmlBody")]
    pub html_body: Option<String>,
    #[serde(default)]
    pub links: Vec<EmailLink>,
    #[serde(default)]
    pub thread: Option<EmailThreadContext>,
    #[serde(default, rename = "calendarAttachments")]
    pub calendar_attachments: Vec<CalendarAttachment>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalendarAttachment {
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailMessageRef {
    #[serde(default, rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "mailboxName")]
    pub mailbox_name: String,
    pub uid: u32,
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailAddress {
    #[serde(default)]
    pub name: Option<String>,
    pub address: String,
    pub domain: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailLink {
    pub url: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailThreadContext {
    #[serde(rename = "threadKey")]
    pub thread_key: String,
    #[serde(default, rename = "normalizedSubject")]
    pub normalized_subject: Option<String>,
    #[serde(rename = "messageCount")]
    pub message_count: u32,
    pub messages: Vec<EmailThreadMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailThreadMessage {
    pub message: EmailMessageRef,
    pub sender: EmailAddress,
    pub subject: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default, rename = "textBody")]
    pub text_body: Option<String>,
    #[serde(default, rename = "htmlBody")]
    pub html_body: Option<String>,
    #[serde(default)]
    pub links: Vec<EmailLink>,
    #[serde(default, rename = "calendarAttachments")]
    pub calendar_attachments: Vec<CalendarAttachment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtractionResult {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub template: String,
    pub confidence: f64,
    pub summary: String,
    #[serde(default)]
    pub fields: serde_json::Value,
    #[serde(default)]
    pub actions: Vec<ExtractionAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row: Option<RowRenderResult>,
    #[serde(
        default,
        rename = "automationHints",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub automation_hints: Vec<serde_json::Value>,
}

impl ExtractionResult {
    pub fn new(result_type: &str, template: &str, confidence: f64, summary: String) -> Self {
        Self {
            schema_version: "1".to_string(),
            result_type: result_type.to_string(),
            template: template.to_string(),
            confidence,
            summary,
            fields: serde_json::Value::Object(Default::default()),
            actions: Vec::new(),
            row: None,
            automation_hints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtractionAction {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RowRenderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<RowBadge>,
    #[serde(rename = "primaryText", skip_serializing_if = "Option::is_none")]
    pub primary_text: Option<String>,
    #[serde(rename = "secondaryText", skip_serializing_if = "Option::is_none")]
    pub secondary_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<RowMetadataItem>,
    #[serde(
        default,
        rename = "quickActions",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub quick_actions: Vec<RowQuickAction>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RowBadge {
    pub label: String,
    pub tone: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RowMetadataItem {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RowQuickAction {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub enabled: bool,
    #[serde(rename = "disabledReason", skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<String>,
}

impl ExtractionAction {
    pub fn open_url(id: &str, label: &str, url: String) -> Self {
        Self {
            id: id.to_string(),
            kind: "open_url".to_string(),
            label: label.to_string(),
            url: Some(url),
            text: None,
            destination: None,
        }
    }

    pub fn suggest_archive(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            kind: "suggest_archive".to_string(),
            label: label.to_string(),
            url: None,
            text: None,
            destination: None,
        }
    }
}

#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let len = len.max(0) as usize;
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr as i32
}

pub unsafe fn read_input<'a>(ptr: i32, len: i32) -> Result<&'a str, String> {
    if ptr < 0 || len < 0 {
        return Err("negative pointer or length".to_string());
    }
    let bytes = std::slice::from_raw_parts(ptr as *const u8, len as usize);
    std::str::from_utf8(bytes).map_err(|e| e.to_string())
}

pub fn parse_email_input(json: &str) -> Result<EmailInput, String> {
    let input: EmailInput = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if let Some(context) = &input.invocation {
        if context.schema_version != "1" || !context.config.is_object() {
            return Err("unsupported or invalid invocation context".into());
        }
    }
    Ok(input)
}

pub fn pack_output_json(value: &impl Serialize) -> i64 {
    let json = serde_json::to_vec(value).unwrap_or_else(|_| {
        br#"{"schemaVersion":"1","type":"plugin.error","template":"error","confidence":0,"summary":"Plugin serialization failed","fields":{},"actions":[]}"#.to_vec()
    });
    pack_bytes(json)
}

pub fn pack_error(summary: &str) -> i64 {
    let mut result = ExtractionResult::new("plugin.error", "error", 0.0, summary.to_string());
    result.fields = serde_json::json!({});
    pack_output_json(&result)
}

pub fn pack_bytes(bytes: Vec<u8>) -> i64 {
    let len = bytes.len() as u32;
    let ptr = bytes.as_ptr() as u32;
    std::mem::forget(bytes);
    ((ptr as i64) << 32) | i64::from(len)
}

#[macro_export]
macro_rules! export_plugin {
    ($handler:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn process(ptr: i32, len: i32) -> i64 {
            let input_json = match $crate::read_input(ptr, len) {
                Ok(value) => value,
                Err(error) => return $crate::pack_error(&format!("Invalid UTF-8 input: {error}")),
            };
            let input = match $crate::parse_email_input(input_json) {
                Ok(value) => value,
                Err(error) => return $crate::pack_error(&format!("Invalid email input: {error}")),
            };
            $crate::pack_output_json(&$handler(input))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_legacy_input_and_versioned_configuration() {
        let fixture = include_str!("../fixtures/email-input.json");
        assert!(parse_email_input(fixture).unwrap().invocation.is_none());
        let mut value: serde_json::Value = serde_json::from_str(fixture).unwrap();
        value["invocation"] =
            serde_json::json!({"schemaVersion": "1", "config": {"previewLength": 50}});
        let input = parse_email_input(&value.to_string()).unwrap();
        assert_eq!(input.invocation.unwrap().config["previewLength"], 50);
        value["invocation"]["schemaVersion"] = serde_json::json!("2");
        assert!(parse_email_input(&value.to_string()).is_err());
        value["invocation"] = serde_json::json!({"schemaVersion": "1", "config": []});
        assert!(parse_email_input(&value.to_string()).is_err());
    }
}

/// Export alongside `export_plugin!`. Both handlers return RecordResult with
/// the same domain identity and semantic update keys across transports.
#[macro_export]
macro_rules! export_refresh {
    ($handler:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn refresh(ptr: i32, len: i32) -> i64 {
            let input = $crate::read_input(ptr, len).ok().and_then(|json| {
                $crate::serde_json::from_str::<$crate::records::RefreshInput>(json).ok()
            });
            match input {
                Some(input) => $crate::pack_output_json(&$handler(input)),
                None => $crate::pack_error("Invalid refresh input"),
            }
        }
    };
}
