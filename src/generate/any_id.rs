use crate::tlv::{encode, tag, with_crc_tag};
use crate::Result;

#[derive(Debug, Clone)]
pub enum ProxyType {
    /// Mobile number
    Msisdn,
    /// National ID or Tax ID
    NatId,
    /// E-Wallet ID
    EWalletId,
    /// Bank Account (Reserved)
    BankAcc,
}

impl ProxyType {
    fn to_code(&self) -> &'static str {
        match self {
            ProxyType::Msisdn => "01",
            ProxyType::NatId => "02",
            ProxyType::EWalletId => "03",
            ProxyType::BankAcc => "04",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnyIdConfig {
    /// Proxy type
    pub proxy_type: ProxyType,
    /// Recipient number
    pub target: String,
    /// Transaction amount
    pub amount: Option<f64>,
}

/// Generate PromptPay AnyID (Tag 29) QR Code
pub fn any_id(config: AnyIdConfig) -> Result<String> {
    let mut target = config.target;

    if matches!(config.proxy_type, ProxyType::Msisdn) {
        // Convert mobile number format: remove leading 0, add 66, pad to 13 digits
        if target.starts_with('0') {
            target = format!("66{}", &target[1..]);
        }
        target = format!("{target:0>13}");
    }

    let tag29_data = vec![
        tag("00", "A000000677010111"),
        tag(config.proxy_type.to_code(), &target),
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

    Ok(with_crc_tag(&encode(&payload), "63", true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_id_msisdn() {
        let config = AnyIdConfig {
            proxy_type: ProxyType::Msisdn,
            target: "0812223333".to_string(),
            amount: None,
        };
        let result = any_id(config).unwrap();
        assert_eq!(
            result,
            "00020101021129370016A0000006770101110113006681222333353037645802TH63041DCF"
        );
    }

    #[test]
    fn test_any_id_msisdn_with_amount() {
        let config = AnyIdConfig {
            proxy_type: ProxyType::Msisdn,
            target: "0812223333".to_string(),
            amount: Some(30.0),
        };
        let result = any_id(config).unwrap();
        assert_eq!(
            result,
            "00020101021229370016A0000006770101110113006681222333353037645802TH540530.0063043CAD"
        );
    }

    #[test]
    fn test_any_id_natid() {
        let config = AnyIdConfig {
            proxy_type: ProxyType::NatId,
            target: "1234567890123".to_string(),
            amount: None,
        };
        let result = any_id(config).unwrap();
        // Just verify it doesn't error and produces a result
        assert!(!result.is_empty());
    }
}
