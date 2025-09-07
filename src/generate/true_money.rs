use crate::tlv::{encode, tag, with_crc_tag};
use crate::utils::encoder::encode_tag81;

#[derive(Debug, Clone)]
pub struct TrueMoneyConfig {
    /// Mobile number
    pub mobile_no: String,
    /// Transaction amount
    pub amount: Option<f64>,
    /// Personal message (Tag 81)
    pub message: Option<String>,
}

/// Generate QR Code for TrueMoney Wallet
///
/// This QR Code can also be scanned with other apps,
/// just like a regular e-Wallet PromptPay QR
/// but `Personal Message (Tag 81)` will be ignored.
pub fn true_money(config: TrueMoneyConfig) -> String {
    let tag29_data = vec![
        tag("00", "A000000677010111"),
        tag("03", &format!("14000{}", config.mobile_no)),
    ];

    let mut payload = vec![
        tag("00", "01"),
        tag("01", if config.amount.is_none() { "11" } else { "12" }),
        tag("29", &encode(&tag29_data)),
        tag("53", "764"),
        tag("58", "TH"),
    ];

    if let Some(amount) = config.amount {
        payload.push(tag("54", &format!("{amount:.2}")));
    }

    if let Some(message) = config.message {
        payload.push(tag("81", &encode_tag81(&message)));
    }

    with_crc_tag(&encode(&payload), "63", true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_money_basic() {
        let config = TrueMoneyConfig {
            mobile_no: "0801111111".to_string(),
            amount: None,
            message: None,
        };
        let result = true_money(config);
        assert_eq!(
            result,
            "00020101021129390016A000000677010111031514000080111111153037645802TH63047C0F"
        );
    }

    #[test]
    fn test_true_money_with_amount_and_message() {
        let config = TrueMoneyConfig {
            mobile_no: "0801111111".to_string(),
            amount: Some(10.05),
            message: Some("Hello World!".to_string()),
        };
        let result = true_money(config);
        assert_eq!(result, "00020101021229390016A000000677010111031514000080111111153037645802TH540510.05814800480065006C006C006F00200057006F0072006C006400216304F5A2");
    }

    #[test]
    fn test_true_money_with_amount_only() {
        let config = TrueMoneyConfig {
            mobile_no: "0801111111".to_string(),
            amount: Some(50.0),
            message: None,
        };
        let result = true_money(config);
        assert!(result.contains("540550.00"));
    }
}
