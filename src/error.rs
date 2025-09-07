use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PromptParseError {
    InvalidPayload,
    InvalidChecksum,
    InvalidTlv,
    TagNotFound(String),
    InvalidProxyType,
    InvalidAmount,
    InvalidMobileNumber,
    MissingRequiredField(String),
    InvalidBarcode,
    ParseError(String),
    EncodingError(String),
}

impl fmt::Display for PromptParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromptParseError::InvalidPayload => write!(f, "Invalid payload format"),
            PromptParseError::InvalidChecksum => write!(f, "Invalid checksum"),
            PromptParseError::InvalidTlv => write!(f, "Invalid TLV data"),
            PromptParseError::TagNotFound(tag) => write!(f, "Tag not found: {tag}"),
            PromptParseError::InvalidProxyType => write!(f, "Invalid proxy type"),
            PromptParseError::InvalidAmount => write!(f, "Invalid amount format"),
            PromptParseError::InvalidMobileNumber => write!(f, "Invalid mobile number format"),
            PromptParseError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {field}")
            }
            PromptParseError::InvalidBarcode => write!(f, "Invalid barcode format"),
            PromptParseError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            PromptParseError::EncodingError(msg) => write!(f, "Encoding error: {msg}"),
        }
    }
}

impl Error for PromptParseError {}
