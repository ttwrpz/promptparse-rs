use crate::tlv::{encode, tag, with_crc_tag};

#[derive(Debug, Clone)]
pub struct SlipVerifyConfig {
    /// Bank code
    pub sending_bank: String,
    /// Transaction reference
    pub trans_ref: String,
}

/// Generate Slip Verify QR Code
///
/// This also called "Mini-QR" that embedded in slip used for verify transactions
pub fn slip_verify(config: SlipVerifyConfig) -> String {
    let tag00_data = vec![
        tag("00", "000001"),
        tag("01", &config.sending_bank),
        tag("02", &config.trans_ref),
    ];

    let payload = vec![tag("00", &encode(&tag00_data)), tag("51", "TH")];

    with_crc_tag(&encode(&payload), "91", true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slip_verify() {
        let config = SlipVerifyConfig {
            sending_bank: "002".to_string(),
            trans_ref: "0002123123121200011".to_string(),
        };
        let result = slip_verify(config);
        assert_eq!(
            result,
            "004000060000010103002021900021231231212000115102TH91049C30"
        );
    }

    #[test]
    fn test_slip_verify_different_bank() {
        let config = SlipVerifyConfig {
            sending_bank: "014".to_string(),
            trans_ref: "00111222233344ABCD12".to_string(),
        };
        let result = slip_verify(config);
        assert!(result.contains("0103014"));
        assert!(result.contains("00111222233344ABCD12"));
    }
}
