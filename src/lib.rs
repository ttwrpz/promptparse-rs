pub mod bot_barcode;
pub mod emvco_qr;
pub mod error;
pub mod generate;
pub mod parser;
pub mod tlv;
pub mod utils;
pub mod validate;

pub use bot_barcode::BotBarcode;
pub use emvco_qr::EmvCoQr;
pub use error::PromptParseError;
pub use parser::{parse, parse_barcode};
pub use tlv::{checksum, decode, encode, get_tag, tag, with_crc_tag, TlvTag};

/// Result type for the library
pub type Result<T> = std::result::Result<T, PromptParseError>;
