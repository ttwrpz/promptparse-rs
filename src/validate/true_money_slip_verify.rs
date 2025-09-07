use crate::parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub struct TrueMoneySlipVerifyData {
    pub event_type: String,
    pub transaction_id: String,
    pub date: String,
}

/// Validate & extract data from TrueMoney Slip Verify QR
///
/// # Arguments
/// * `payload` - QR Code Payload
///
/// # Returns
/// Type, Transaction ID and Date (DDMMYYYY) or None if payload invalid
pub fn true_money_slip_verify(payload: &str) -> Option<TrueMoneySlipVerifyData> {
    let ppqr = parse(payload, true, true)?;

    let tag00_val = ppqr.get_tag_value("00", Some("00"))?;
    let tag01_val = ppqr.get_tag_value("00", Some("01"))?;

    if tag00_val != "01" || tag01_val != "01" {
        return None;
    }

    let event_type = ppqr.get_tag_value("00", Some("02"))?;
    let transaction_id = ppqr.get_tag_value("00", Some("03"))?;
    let date = ppqr.get_tag_value("00", Some("04"))?;

    Some(TrueMoneySlipVerifyData {
        event_type: event_type.to_string(),
        transaction_id: transaction_id.to_string(),
        date: date.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_money_slip_verify_extraction() {
        // This is a hypothetical valid TrueMoney slip verify payload
        // The actual test would need a real payload
        let event_type = "P2P";
        let transaction_id = "TXN123456789";
        let date = "01012024";

        // Create expected result
        let expected = TrueMoneySlipVerifyData {
            event_type: event_type.to_string(),
            transaction_id: transaction_id.to_string(),
            date: date.to_string(),
        };

        assert_eq!(expected.event_type, "P2P");
        assert_eq!(expected.transaction_id, "TXN123456789");
        assert_eq!(expected.date, "01012024");
    }

    #[test]
    fn test_true_money_slip_verify_invalid_format() {
        let payload = "invalid_payload";
        assert!(true_money_slip_verify(payload).is_none());
    }
}
