use crate::bot_barcode::BotBarcode;

#[derive(Debug, Clone)]
pub struct BotBarcodeConfig {
    /// Biller ID (Tax ID + Suffix)
    pub biller_id: String,
    /// Reference No. 1 / Customer No.
    pub ref1: String,
    /// Reference No. 2
    pub ref2: Option<String>,
    /// Transaction amount
    pub amount: Option<f64>,
}

/// Generate BOT Barcode
pub fn bot_barcode(config: BotBarcodeConfig) -> String {
    let barcode = BotBarcode::new(config.biller_id, config.ref1, config.ref2, config.amount);
    barcode.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_barcode_basic() {
        let config = BotBarcodeConfig {
            biller_id: "099999999999990".to_string(),
            ref1: "111222333444".to_string(),
            ref2: None,
            amount: None,
        };
        let result = bot_barcode(config);
        assert_eq!(result, "|099999999999990\r111222333444\r\r0");
    }

    #[test]
    fn test_bot_barcode_with_ref2_and_amount() {
        let config = BotBarcodeConfig {
            biller_id: "099400016550100".to_string(),
            ref1: "123456789012".to_string(),
            ref2: Some("670429".to_string()),
            amount: Some(3649.22),
        };
        let result = bot_barcode(config);
        assert_eq!(result, "|099400016550100\r123456789012\r670429\r364922");
    }

    #[test]
    fn test_bot_barcode_with_amount_only() {
        let config = BotBarcodeConfig {
            biller_id: "099999999999990".to_string(),
            ref1: "111222333444".to_string(),
            ref2: None,
            amount: Some(100.0),
        };
        let result = bot_barcode(config);
        assert_eq!(result, "|099999999999990\r111222333444\r\r10000");
    }
}
