use crate::tlv::{encode, tag, with_crc_tag};
use crate::Result;

#[derive(Debug, Clone)]
pub struct BillPaymentConfig {
    /// Biller ID (National ID or Tax ID + Suffix)
    pub biller_id: String,
    /// Transaction amount
    pub amount: Option<f64>,
    /// Reference 1
    pub ref1: String,
    /// Reference 2
    pub ref2: Option<String>,
    /// (Undocumented) Reference 3
    pub ref3: Option<String>,
}

/// Generate PromptPay Bill Payment (Tag 30) QR Code
pub fn bill_payment(config: BillPaymentConfig) -> Result<String> {
    let mut tag30_data = vec![
        tag("00", "A000000677010112"),
        tag("01", &config.biller_id),
        tag("02", &config.ref1),
    ];

    if let Some(ref2) = &config.ref2 {
        tag30_data.push(tag("03", ref2));
    }

    let mut payload = vec![
        tag("00", "01"),
        tag("01", if config.amount.is_none() { "11" } else { "12" }),
        tag("30", &encode(&tag30_data)),
        tag("53", "764"),
        tag("58", "TH"),
    ];

    if let Some(amount) = config.amount {
        payload.push(tag("54", &format!("{amount:.2}")));
    }

    if let Some(ref3) = config.ref3 {
        let tag62_data = vec![tag("07", &ref3)];
        payload.push(tag("62", &encode(&tag62_data)));
    }

    Ok(with_crc_tag(&encode(&payload), "63", true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_payment_basic() {
        let config = BillPaymentConfig {
            biller_id: "0112233445566".to_string(),
            amount: None,
            ref1: "CUSTOMER001".to_string(),
            ref2: None,
            ref3: None,
        };
        let result = bill_payment(config).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_bill_payment_with_all_refs() {
        let config = BillPaymentConfig {
            biller_id: "0112233445566".to_string(),
            amount: None,
            ref1: "CUSTOMER001".to_string(),
            ref2: Some("INV001".to_string()),
            ref3: Some("SCB".to_string()),
        };
        let result = bill_payment(config).unwrap();
        assert_eq!(result, "00020101021130620016A000000677010112011301122334455660211CUSTOMER0010306INV00153037645802TH62070703SCB6304780E");
    }

    #[test]
    fn test_bill_payment_with_amount() {
        let config = BillPaymentConfig {
            biller_id: "0112233445566".to_string(),
            amount: Some(100.50),
            ref1: "CUSTOMER001".to_string(),
            ref2: None,
            ref3: None,
        };
        let result = bill_payment(config).unwrap();
        assert!(result.contains("5406100.50"));
    }
}
