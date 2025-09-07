use crate::generate::bill_payment;
use crate::generate::BillPaymentConfig;
use crate::Result;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BotBarcode {
    pub biller_id: String,
    pub ref1: String,
    pub ref2: Option<String>,
    pub amount: Option<f64>,
}

impl BotBarcode {
    pub fn new(biller_id: String, ref1: String, ref2: Option<String>, amount: Option<f64>) -> Self {
        Self {
            biller_id,
            ref1,
            ref2,
            amount,
        }
    }

    pub fn from_string(payload: &str) -> Option<Self> {
        if !payload.starts_with('|') {
            return None;
        }

        let data: Vec<&str> = payload[1..].split('\r').collect();
        if data.len() != 4 {
            return None;
        }

        let biller_id = data[0].to_string();
        let ref1 = data[1].to_string();
        let ref2 = if data[2].is_empty() {
            None
        } else {
            Some(data[2].to_string())
        };

        let amount = if data[3] == "0" {
            None
        } else {
            data[3].parse::<i32>().ok().map(|amt| (amt as f64) / 100.0)
        };

        Some(Self::new(biller_id, ref1, ref2, amount))
    }

    /// Converts BOT Barcode to PromptPay QR Tag 30 (Bill Payment)
    ///
    /// This method works for some biller, depends on destination bank
    pub fn to_qr_tag30(&self) -> Result<String> {
        let config = BillPaymentConfig {
            biller_id: self.biller_id.clone(),
            amount: self.amount,
            ref1: self.ref1.clone(),
            ref2: self.ref2.clone(),
            ref3: None,
        };

        bill_payment(config)
    }
}

impl fmt::Display for BotBarcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount_str = self
            .amount
            .map(|amt| ((amt * 100.0) as i32).to_string())
            .unwrap_or_else(|| "0".to_string());

        write!(
            f,
            "|{}\r{}\r{}\r{}",
            self.biller_id,
            self.ref1,
            self.ref2.as_deref().unwrap_or(""),
            amount_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_barcode_from_string() {
        let payload = "|099999999999990\r111222333444\r\r0";
        let barcode = BotBarcode::from_string(payload).unwrap();

        assert_eq!(barcode.biller_id, "099999999999990");
        assert_eq!(barcode.ref1, "111222333444");
        assert_eq!(barcode.ref2, None);
        assert_eq!(barcode.amount, None);
    }

    #[test]
    fn test_bot_barcode_with_ref2_and_amount() {
        let payload = "|099400016550100\r123456789012\r670429\r364922";
        let barcode = BotBarcode::from_string(payload).unwrap();

        assert_eq!(barcode.biller_id, "099400016550100");
        assert_eq!(barcode.ref1, "123456789012");
        assert_eq!(barcode.ref2, Some("670429".to_string()));
        assert_eq!(barcode.amount, Some(3649.22));
    }

    #[test]
    fn test_bot_barcode_to_string() {
        let barcode = BotBarcode::new(
            "099999999999990".to_string(),
            "111222333444".to_string(),
            None,
            None,
        );

        assert_eq!(barcode.to_string(), "|099999999999990\r111222333444\r\r0");
    }

    #[test]
    fn test_bot_barcode_invalid_format() {
        assert!(BotBarcode::from_string("invalid").is_none());
        assert!(BotBarcode::from_string("|too\rfew\rparts").is_none());
    }
}
