use crate::parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub struct SlipVerifyData {
    pub sending_bank: String,
    pub trans_ref: String,
}

/// Validate & extract data from Slip Verify QR (for use with Bank Open API)
///
/// # Arguments
/// * `payload` - QR Code Payload
/// * `crc_auto_fix` - Attempt to fix bad checksum from bank app
///
/// # Returns
/// Bank code and Transaction reference or None if payload invalid
pub fn slip_verify(payload: &str, crc_auto_fix: bool) -> Option<SlipVerifyData> {
    let mut payload = payload.to_string();

    if crc_auto_fix {
        if let Some(idx) = payload.rfind("9104") {
            let crc_start = idx + 4;
            if crc_start < payload.len() {
                let crc = &payload[crc_start..];
                if !crc.is_empty() && crc.len() < 4 {
                    // Pad the CRC with leading zeros to make it 4 characters
                    payload = format!("{}{:0>4}", &payload[..crc_start], crc);
                }
            }
        }
    }

    let ppqr = parse(&payload, true, true)?;

    let api_type = ppqr.get_tag_value("00", Some("00"))?;
    if api_type != "000001" {
        return None;
    }

    let sending_bank = ppqr.get_tag_value("00", Some("01"))?;
    let trans_ref = ppqr.get_tag_value("00", Some("02"))?;

    Some(SlipVerifyData {
        sending_bank: sending_bank.to_string(),
        trans_ref: trans_ref.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slip_verify_valid() {
        let payload = "004100060000010103014022000111222233344ABCD126304BA3C";
        let result = slip_verify(payload, true).unwrap();
        assert_eq!(result.sending_bank, "014");
        assert_eq!(result.trans_ref, "00111222233344ABCD12");
    }

    #[test]
    fn test_slip_verify_invalid() {
        let payload =
            "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
        assert!(slip_verify(payload, true).is_none());
    }

    #[test]
    fn test_slip_verify_invalid_api_type() {
        // This would be a payload with wrong API type
        let payload = "004000060000020103014022000111222233344ABCD125102TH9104XXXX";
        assert!(slip_verify(payload, true).is_none());
    }
}
