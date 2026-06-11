use crate::parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub struct BillPaymentData {
    /// Biller ID
    pub biller_id: String,
    /// Reference 1 (required)
    pub ref1: String,
    /// Reference 2 (if present)
    pub ref2: Option<String>,
    /// (Undocumented) Reference 3 (if present)
    pub ref3: Option<String>,
    /// Transaction amount (if present)
    pub amount: Option<f64>,
}

/// Validate & extract data from PromptPay Bill Payment (Tag 30) QR Code
///
/// # Arguments
/// * `payload` - QR Code Payload
///
/// # Returns
/// Biller ID, references and optional amount or None if payload invalid
pub fn bill_payment(payload: &str) -> Option<BillPaymentData> {
    let ppqr = parse(payload, true, true)?;

    let aid_type = ppqr.get_tag_value("00", None)?;
    let tag30_aid = ppqr.get_tag_value("30", Some("00"))?;

    if aid_type != "01" || tag30_aid != "A000000677010112" {
        return None;
    }

    let biller_id = ppqr
        .get_tag_value("30", Some("01"))
        .filter(|s| !s.is_empty())?;
    let ref1 = ppqr
        .get_tag_value("30", Some("02"))
        .filter(|s| !s.is_empty())?;

    let ref2 = ppqr.get_tag_value("30", Some("03")).map(|s| s.to_string());
    let ref3 = ppqr.get_tag_value("62", Some("07")).map(|s| s.to_string());

    let amount = ppqr
        .get_tag_value("54", None)
        .and_then(|s| s.parse::<f64>().ok());

    Some(BillPaymentData {
        biller_id: biller_id.to_string(),
        ref1: ref1.to_string(),
        ref2,
        ref3,
        amount,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate::any_id::{any_id as generate_any_id, AnyIdConfig, ProxyType};
    use crate::generate::bill_payment::{bill_payment as generate_bill_payment, BillPaymentConfig};

    #[test]
    fn test_bill_payment_minimal() {
        let payload = generate_bill_payment(BillPaymentConfig {
            biller_id: "0112233445566".to_string(),
            amount: None,
            ref1: "CUSTOMER001".to_string(),
            ref2: None,
            ref3: None,
        })
        .unwrap();

        let result = bill_payment(&payload).unwrap();
        assert_eq!(result.biller_id, "0112233445566");
        assert_eq!(result.ref1, "CUSTOMER001");
        assert_eq!(result.ref2, None);
        assert_eq!(result.ref3, None);
        assert_eq!(result.amount, None);
    }

    #[test]
    fn test_bill_payment_complete() {
        let payload = generate_bill_payment(BillPaymentConfig {
            biller_id: "0112233445566".to_string(),
            amount: Some(100.50),
            ref1: "CUSTOMER001".to_string(),
            ref2: Some("INV001".to_string()),
            ref3: Some("SCB".to_string()),
        })
        .unwrap();

        let result = bill_payment(&payload).unwrap();
        assert_eq!(result.biller_id, "0112233445566");
        assert_eq!(result.ref1, "CUSTOMER001");
        assert_eq!(result.ref2, Some("INV001".to_string()));
        assert_eq!(result.ref3, Some("SCB".to_string()));
        assert_eq!(result.amount, Some(100.50));
    }

    #[test]
    fn test_bill_payment_invalid_payload() {
        assert!(bill_payment("invalid_payload").is_none());
    }

    #[test]
    fn test_bill_payment_rejects_any_id() {
        // An AnyID (Tag 29) payload must not validate as Bill Payment
        let payload = generate_any_id(AnyIdConfig {
            proxy_type: ProxyType::Msisdn,
            target: "0812223333".to_string(),
            amount: None,
        })
        .unwrap();

        assert!(bill_payment(&payload).is_none());
    }
}
