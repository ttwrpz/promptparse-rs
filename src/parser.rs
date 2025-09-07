use crate::bot_barcode::BotBarcode;
use crate::emvco_qr::EmvCoQr;
use crate::tlv::{checksum, decode};

/// Parse any EMVCo-compatible QR Code data string
///
/// # Arguments
/// * `payload` - QR Code data string from the scanner
/// * `strict` - Validate CRC checksum before parsing the entire string
/// * `sub_tags` - Parse TLV Sub-tags (If exists)
///
/// # Returns
/// QR Instance with TLV Tags
pub fn parse(payload: &str, strict: bool, sub_tags: bool) -> Option<EmvCoQr> {
    if payload.len() < 5 {
        return None;
    }

    if !payload.chars().take(4).all(|c| c.is_ascii_digit()) {
        return None;
    }

    if strict {
        let expected = payload[payload.len() - 4..].to_uppercase();
        let calculated = checksum(&payload[..payload.len() - 4], true);
        if expected != calculated {
            return None;
        }
    }

    let mut tags = decode(payload).ok()?;
    if tags.is_empty() {
        return None;
    }

    if sub_tags {
        for tag in &mut tags {
            if tag.value.len() >= 5 && tag.value.chars().take(4).all(|c| c.is_ascii_digit()) {
                if let Ok(sub) = decode(&tag.value) {
                    if sub
                        .iter()
                        .all(|val| val.length > 0 && val.length == val.value.len())
                    {
                        tag.sub_tags = Some(sub);
                    }
                }
            }
        }
    }

    Some(EmvCoQr::new(payload.to_string(), tags))
}

/// Parse barcode data string (BOT Barcode Standard)
///
/// # Arguments
/// * `payload` - Barcode data string from the scanner
///
/// # Returns
/// BOT Barcode Instance
pub fn parse_barcode(payload: &str) -> Option<BotBarcode> {
    BotBarcode::from_string(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_payload() {
        let payload = "000411110104222202043333";
        let result = parse(payload, false, true).unwrap();
        assert_eq!(result.get_tags().len(), 3);
    }

    #[test]
    fn test_parse_invalid_payload() {
        let payload = "AAAA0000";
        assert!(parse(payload, false, true).is_none());
    }

    #[test]
    fn test_parse_with_strict_mode_valid() {
        let payload =
            "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
        let result = parse(payload, true, true);
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_with_strict_mode_invalid() {
        let payload =
            "00020101021229370016A0000006770101110113006680111111153037645802TH540520.156304FFFF";
        assert!(parse(payload, true, true).is_none());
    }

    #[test]
    fn test_parse_barcode_valid() {
        let payload = "|099999999999990\r111222333444\r\r0";
        let result = parse_barcode(payload).unwrap();
        assert_eq!(result.biller_id, "099999999999990");
    }

    #[test]
    fn test_parse_barcode_invalid() {
        let payload = "invalid barcode";
        assert!(parse_barcode(payload).is_none());
    }
}
