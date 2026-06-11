use crate::generate::any_id::ProxyType;
use crate::parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub struct AnyIdData {
    /// Proxy type
    pub r#type: ProxyType,
    /// Target identifier (e.g. mobile number or National ID)
    pub target: String,
    /// Transaction amount (if present)
    pub amount: Option<f64>,
}

/// Validate & extract data from PromptPay AnyID (Tag 29) QR Code
///
/// # Arguments
/// * `payload` - QR Code Payload
///
/// # Returns
/// Proxy type, target identifier and optional amount or None if payload invalid
pub fn any_id(payload: &str) -> Option<AnyIdData> {
    let ppqr = parse(payload, true, true)?;

    let aid_type = ppqr.get_tag_value("00", None)?;
    let tag29_aid = ppqr.get_tag_value("29", Some("00"))?;

    if aid_type != "01" || tag29_aid != "A000000677010111" {
        return None;
    }

    // Resolve proxy type from the sub-tag ID that is present
    let (r#type, mut target) = [
        ProxyType::Msisdn,
        ProxyType::NatId,
        ProxyType::EWalletId,
        ProxyType::BankAcc,
    ]
    .into_iter()
    .find_map(|proxy_type| {
        ppqr.get_tag_value("29", Some(proxy_type.to_code()))
            .map(|value| (proxy_type, value.to_string()))
    })?;

    // Normalise MSISDN back to local format (0XXXXXXXXX)
    if matches!(r#type, ProxyType::Msisdn) && target.len() == 13 && target.starts_with("0066") {
        target = format!("0{}", &target[target.len() - 9..]);
    }

    let amount = ppqr
        .get_tag_value("54", None)
        .and_then(|s| s.parse::<f64>().ok());

    Some(AnyIdData {
        r#type,
        target,
        amount,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate::any_id::{any_id as generate_any_id, AnyIdConfig};
    use crate::generate::bill_payment::{bill_payment as generate_bill_payment, BillPaymentConfig};

    #[test]
    fn test_any_id_msisdn() {
        let payload = generate_any_id(AnyIdConfig {
            proxy_type: ProxyType::Msisdn,
            target: "0812223333".to_string(),
            amount: None,
        })
        .unwrap();

        let result = any_id(&payload).unwrap();
        assert_eq!(result.r#type, ProxyType::Msisdn);
        assert_eq!(result.target, "0812223333");
        assert_eq!(result.amount, None);
    }

    #[test]
    fn test_any_id_msisdn_with_amount() {
        let payload = generate_any_id(AnyIdConfig {
            proxy_type: ProxyType::Msisdn,
            target: "0812223333".to_string(),
            amount: Some(30.0),
        })
        .unwrap();

        let result = any_id(&payload).unwrap();
        assert_eq!(result.r#type, ProxyType::Msisdn);
        assert_eq!(result.target, "0812223333");
        assert_eq!(result.amount, Some(30.0));
    }

    #[test]
    fn test_any_id_natid() {
        let payload = generate_any_id(AnyIdConfig {
            proxy_type: ProxyType::NatId,
            target: "1234567890123".to_string(),
            amount: None,
        })
        .unwrap();

        let result = any_id(&payload).unwrap();
        assert_eq!(result.r#type, ProxyType::NatId);
        assert_eq!(result.target, "1234567890123");
        assert_eq!(result.amount, None);
    }

    #[test]
    fn test_any_id_invalid_payload() {
        assert!(any_id("invalid_payload").is_none());
    }

    #[test]
    fn test_any_id_rejects_bill_payment() {
        // A Bill Payment (Tag 30) payload must not validate as AnyID
        let payload = generate_bill_payment(BillPaymentConfig {
            biller_id: "0112233445566".to_string(),
            amount: None,
            ref1: "CUSTOMER001".to_string(),
            ref2: None,
            ref3: None,
        })
        .unwrap();

        assert!(any_id(&payload).is_none());
    }
}
