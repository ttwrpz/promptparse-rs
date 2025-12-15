use crate::parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub struct BcelOneProofData {
    pub r#type: Option<String>,
    pub ticket: Option<String>,
    pub fccref: Option<String>,
}

/// Validate & extract data from BCEL OneProof QR
///
/// # Arguments
/// * `payload` - QR Code Payload
///
/// # Returns
/// Type, Ticket No. and Reference No. or None if payload invalid
pub fn bcel_one_proof(payload: &str) -> Option<BcelOneProofData> {
    let ppqr = parse(payload, true, true)?;

    let r#type = ppqr.get_tag_value("33", Some("02")).map(|s| s.to_string());
    let ticket = ppqr.get_tag_value("33", Some("03")).map(|s| s.to_string());
    let fccref = ppqr.get_tag_value("33", Some("04")).map(|s| s.to_string());

    let tag_00 = ppqr.get_tag_value("00", None)?;
    let tag_01 = ppqr.get_tag_value("01", None)?;
    let tag_33_00 = ppqr.get_tag_value("33", Some("00"))?;

    if tag_00 != "01" || tag_01 != "11" {
        return None;
    }

    if tag_33_00 != "BCEL" && tag_33_00 != "ONEPROOF" {
        return None;
    }

    Some(BcelOneProofData {
        r#type,
        ticket,
        fccref,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcel_one_proof_structure() {
        // Test that the data structure can be created
        let data = BcelOneProofData {
            r#type: Some("TRANSFER".to_string()),
            ticket: Some("TKT123456".to_string()),
            fccref: Some("REF789012".to_string()),
        };

        assert_eq!(data.r#type, Some("TRANSFER".to_string()));
        assert_eq!(data.ticket, Some("TKT123456".to_string()));
        assert_eq!(data.fccref, Some("REF789012".to_string()));
    }

    #[test]
    fn test_bcel_one_proof_with_none_values() {
        // Test that optional fields can be None
        let data = BcelOneProofData {
            r#type: None,
            ticket: Some("TKT123456".to_string()),
            fccref: None,
        };

        assert_eq!(data.r#type, None);
        assert_eq!(data.ticket, Some("TKT123456".to_string()));
        assert_eq!(data.fccref, None);
    }

    #[test]
    fn test_bcel_one_proof_invalid_payload() {
        // Test with an invalid payload
        let payload = "invalid_payload";
        let result = bcel_one_proof(payload);
        assert!(result.is_none());
    }

    #[test]
    fn test_bcel_one_proof_wrong_tag_00() {
        // Test with wrong tag 00 value (should fail validation)
        // This would need a properly formatted QR with wrong tag 00
        let payload = "00020101021229370016A0000006770101110113006680111111153037645802TH63041DCF";
        let result = bcel_one_proof(payload);
        assert!(result.is_none());
    }
}
