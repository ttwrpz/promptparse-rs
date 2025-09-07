// Additional library-level tests that complement the integration tests

use promptparse::*;

#[test]
fn test_library_public_api() {
    // Test that all main functions are accessible from the library root

    // Parse functions
    let _parse_fn = parse;
    let _parse_barcode_fn = parse_barcode;

    // TLV functions
    let _decode_fn = decode;
    let _encode_fn = encode;
    let _checksum_fn = checksum;
    let _with_crc_tag_fn = with_crc_tag;
    let _get_tag_fn = get_tag;
    let _tag_fn = tag;

    // Types should be accessible
    let _tlv_tag = TlvTag::new("00".to_string(), "01".to_string());
    let _bot_barcode = BotBarcode::new("123".to_string(), "ref1".to_string(), None, None);
}

#[test]
fn test_error_types() {
    use promptparse::PromptParseError;

    // Test that error types are properly exposed
    let _error = PromptParseError::InvalidPayload;
    let _error = PromptParseError::InvalidChecksum;
    let _error = PromptParseError::InvalidTlv;
}

#[test]
fn test_tlv_operations() {
    use promptparse::{checksum, decode, encode, tag, with_crc_tag};

    // Test basic TLV operations
    let tags = vec![tag("00", "01"), tag("01", "11"), tag("53", "764")];

    let encoded = encode(&tags);
    let decoded = decode(&encoded).unwrap();

    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].id, "00");
    assert_eq!(decoded[0].value, "01");
    assert_eq!(decoded[1].id, "01");
    assert_eq!(decoded[1].value, "11");
    assert_eq!(decoded[2].id, "53");
    assert_eq!(decoded[2].value, "764");

    // Test checksum
    let crc = checksum(&encoded, true);
    assert_eq!(crc.len(), 4);

    // Test with CRC tag
    let with_crc = with_crc_tag(&encoded, "63", true);
    assert_eq!(with_crc, "00020101021153037646304956E");
}

#[test]
fn test_emvco_qr_methods() {
    let payload = "000201010211";
    let tags = decode(payload).unwrap();
    let qr = EmvCoQr::new(payload.to_string(), tags);

    // Test all public methods
    assert_eq!(qr.get_payload(), payload);
    assert_eq!(qr.get_tags().len(), 2);
    assert_eq!(qr.get_tag_value("00", None), Some("01"));
    assert_eq!(qr.get_tag_value("01", None), Some("11"));
    assert!(qr.get_tag("00", None).is_some());

    // Test validation (need a proper CRC to pass)
    let result = qr.validate("63");
    assert!(!result, "Expected invalid CRC to return false");
}

#[test]
fn test_bot_barcode_methods() {
    let barcode = BotBarcode::new(
        "123456789012345".to_string(),
        "REF001".to_string(),
        Some("REF002".to_string()),
        Some(100.0),
    );

    // Test serialization
    let barcode_str = barcode.to_string();
    assert!(barcode_str.starts_with('|'));
    assert!(barcode_str.contains("123456789012345"));
    assert!(barcode_str.contains("REF001"));
    assert!(barcode_str.contains("REF002"));
    assert!(barcode_str.contains("10000")); // 100.0 * 100

    // Test deserialization
    let parsed = BotBarcode::from_string(&barcode_str).unwrap();
    assert_eq!(parsed.biller_id, barcode.biller_id);
    assert_eq!(parsed.ref1, barcode.ref1);
    assert_eq!(parsed.ref2, barcode.ref2);
    assert_eq!(parsed.amount, barcode.amount);

    // Test QR conversion
    let _qr_result = barcode.to_qr_tag30();
}

#[test]
fn test_generate_module_accessibility() {
    use promptparse::generate::*;

    // Test that all generate functions are accessible
    let any_id_config = AnyIdConfig {
        proxy_type: ProxyType::Msisdn,
        target: "0812345678".to_string(),
        amount: None,
    };
    let _result = any_id(any_id_config);

    let bill_config = BillPaymentConfig {
        biller_id: "123456789012345".to_string(),
        amount: None,
        ref1: "REF001".to_string(),
        ref2: None,
        ref3: None,
    };
    let _result = bill_payment(bill_config);

    let true_money_config = TrueMoneyConfig {
        mobile_no: "0812345678".to_string(),
        amount: None,
        message: None,
    };
    let _result = true_money(true_money_config);

    let slip_config = SlipVerifyConfig {
        sending_bank: "014".to_string(),
        trans_ref: "REF123456789".to_string(),
    };
    let _result = slip_verify(slip_config);

    let bot_config = BotBarcodeConfig {
        biller_id: "123456789012345".to_string(),
        ref1: "REF001".to_string(),
        ref2: None,
        amount: None,
    };
    let _result = bot_barcode(bot_config);
}

#[test]
fn test_validate_module_accessibility() {
    use promptparse::validate;

    // Test validate functions (they will return None for invalid data, but should be callable)
    let _result = validate::slip_verify("invalid");
    let _result = validate::true_money_slip_verify("invalid");
}

#[test]
fn test_proxy_type_enum() {
    use promptparse::generate::ProxyType;

    // Test that all enum variants are accessible
    let _msisdn = ProxyType::Msisdn;
    let _nat_id = ProxyType::NatId;
    let _ewallet_id = ProxyType::EWalletId;
    let _bank_acc = ProxyType::BankAcc;
}

#[test]
fn test_data_structures() {
    use promptparse::validate::{SlipVerifyData, TrueMoneySlipVerifyData};

    // Test that validation result structures are accessible
    let slip_data = SlipVerifyData {
        sending_bank: "014".to_string(),
        trans_ref: "REF123".to_string(),
    };
    assert_eq!(slip_data.sending_bank, "014");
    assert_eq!(slip_data.trans_ref, "REF123");

    let truemoney_data = TrueMoneySlipVerifyData {
        event_type: "P2P".to_string(),
        transaction_id: "TXN123".to_string(),
        date: "01012024".to_string(),
    };
    assert_eq!(truemoney_data.event_type, "P2P");
    assert_eq!(truemoney_data.transaction_id, "TXN123");
    assert_eq!(truemoney_data.date, "01012024");
}
