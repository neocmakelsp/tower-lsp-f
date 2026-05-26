//! extension for [lsp_types::Message]
//
//
use lsp_types::Message;
/// extension for [lsp_types::Message]
pub trait MessageEx {
    /// Get the content
    fn content_string(&self) -> &str;
}

impl MessageEx for Message {
    fn content_string(&self) -> &str {
        match self {
            Message::String(content) => content.as_str(),
            Message::MarkupContent(content) => &content.value,
        }
    }
}
