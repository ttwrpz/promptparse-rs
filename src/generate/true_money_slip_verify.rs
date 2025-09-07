use crate::tlv::{encode, tag, with_crc_tag};

#[derive(Debug, Clone)]
pub struct TrueMoneySlipVerifyConfig {
    /// Event Type (Example: P2P)
    pub event_type: String,
    /// Transaction ID
    pub transaction_id: String,
    /// Date (DDMMYYYY)
    pub date: String,
}

/// Generate TrueMoney Slip Verify QR Code
///
/// Same as a regular Slip Verify QR but with some differences
/// - Tag 00 and 01 are set to '01'
/// - Tag 51 does not exist
/// - Additional tags that are TrueMoney-specific
/// - CRC checksum are case-sensitive
pub fn true_money_slip_verify(config: TrueMoneySlipVerifyConfig) -> String {
    let tag00_data = vec![
        tag("00", "01"),
        tag("01", "01"),
        tag("02", &config.event_type),
        tag("03", &config.transaction_id),
        tag("04", &config.date),
    ];

    let payload = vec![tag("00", &encode(&tag00_data))];

    with_crc_tag(&encode(&payload), "91", false) // Note: case-sensitive (false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_money_slip_verify() {
        let config = TrueMoneySlipVerifyConfig {
            event_type: "P2P".to_string(),
            transaction_id: "TXN123456789".to_string(),
            date: "01012024".to_string(),
        };
        let result = true_money_slip_verify(config);

        // Verify it contains the expected components
        assert!(result.contains("00")); // Main tag
        assert!(result.contains("91")); // CRC tag
        assert!(!result.is_empty());
    }

    #[test]
    fn test_true_money_slip_verify_structure() {
        let config = TrueMoneySlipVerifyConfig {
            event_type: "P2P".to_string(),
            transaction_id: "TXN123456789".to_string(),
            date: "31122023".to_string(),
        };
        let result = true_money_slip_verify(config);

        // Should start with tag 00 and end with 91 + 04 + CRC
        assert!(result.starts_with("00"));
        assert!(result.contains("9104"));
    }
}
