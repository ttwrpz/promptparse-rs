use promptparse::{
    generate::{
        any_id, bill_payment, bot_barcode, slip_verify, true_money, true_money_slip_verify,
    },
    generate::{
        AnyIdConfig, BillPaymentConfig, BotBarcodeConfig, ProxyType, SlipVerifyConfig,
        TrueMoneyConfig, TrueMoneySlipVerifyConfig,
    },
    parse, parse_barcode, validate,
};

#[test]
fn test_invalid_string_passed_to_parser() {
    assert!(parse("AAAA0000", false, true).is_none());
}

#[test]
fn test_parse_tlv_and_get_tag_count() {
    let result = parse("000411110104222202043333", false, true).unwrap();
    assert_eq!(result.get_tags().len(), 3);
}

#[test]
fn test_parse_tlv_and_get_one_tag() {
    let result = parse("000411110104222202043333", false, true).unwrap();
    assert_eq!(result.get_tag_value("01", None).unwrap(), "2222");
}

#[test]
fn test_parse_payload_strict_with_invalid_checksum() {
    let payload =
        "00020101021229370016A0000006770101110113006680111111153037645802TH540520.156304FFFF";
    assert!(parse(payload, true, true).is_none());
}

#[test]
fn test_parse_payload_strict_with_valid_checksum_and_get_tag_value() {
    let payload =
        "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
    let result = parse(payload, true, true).unwrap();
    assert_eq!(
        result.get_tag_value("29", Some("01")).unwrap(),
        "0066801111111"
    );
}

#[test]
fn test_generate_any_id() {
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
fn test_generate_any_id_with_amount() {
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
fn test_generate_slip_verify() {
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
fn test_generate_true_money_qr() {
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
fn test_generate_true_money_qr_with_amount_and_message() {
    let config = TrueMoneyConfig {
        mobile_no: "0801111111".to_string(),
        amount: Some(10.05),
        message: Some("Hello World!".to_string()),
    };
    let result = true_money(config);
    assert_eq!(result, "00020101021229390016A000000677010111031514000080111111153037645802TH540510.05814800480065006C006C006F00200057006F0072006C006400216304F5A2");
}

#[test]
fn test_generate_bill_payment_with_ref3() {
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
fn test_generate_bot_barcode() {
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
fn test_generate_bot_barcode_with_ref2_and_amount() {
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
fn test_validate_checksum_tag() {
    let payload =
        "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
    let result = parse(payload, false, true).unwrap();
    assert!(result.validate("63"));
}

#[test]
fn test_validate_slip_verify_valid() {
    let payload = "004100060000010103014022000111222233344ABCD125102TH910417DF";
    let result = validate::slip_verify(payload).unwrap();
    assert_eq!(result.sending_bank, "014");
    assert_eq!(result.trans_ref, "00111222233344ABCD12");
}

#[test]
fn test_validate_slip_verify_invalid() {
    let payload =
        "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
    assert!(validate::slip_verify(payload).is_none());
}

#[test]
fn test_convert_bot_barcode_to_bill_payment_valid() {
    let barcode = parse_barcode("|099999999999990\r111222333444\r\r0").unwrap();
    let result = barcode.to_qr_tag30().unwrap();
    assert_eq!(result, "00020101021130550016A0000006770101120115099999999999990021211122233344453037645802TH63043EE7");
}

#[test]
fn test_convert_bot_barcode_to_bill_payment_valid_with_ref2_and_amount() {
    let barcode = parse_barcode("|099400016550100\r123456789012\r670429\r364922").unwrap();
    let result = barcode.to_qr_tag30().unwrap();
    assert_eq!(result, "00020101021230650016A00000067701011201150994000165501000212123456789012030667042953037645802TH54073649.2263044534");
}

#[test]
fn test_convert_bot_barcode_to_bill_payment_invalid_wrong_payload() {
    let result = parse_barcode("00020101021230650016A00000067701011201150994000165501000212123456789012030667042953037645802TH54073649.2263044534");
    assert!(result.is_none());
}

#[test]
fn test_convert_bot_barcode_to_bill_payment_invalid_data_loss() {
    let result = parse_barcode("|099400016550100\r123456789012\r670429");
    assert!(result.is_none());
}

#[test]
fn test_generate_true_money_slip_verify() {
    let config = TrueMoneySlipVerifyConfig {
        event_type: "P2P".to_string(),
        transaction_id: "TXN123456789".to_string(),
        date: "01012024".to_string(),
    };
    let result = true_money_slip_verify(config);

    // Should contain the basic structure
    assert!(result.contains("00"));
    assert!(result.contains("91"));
    assert!(!result.is_empty());
}

#[test]
fn test_any_id_with_national_id() {
    let config = AnyIdConfig {
        proxy_type: ProxyType::NatId,
        target: "1234567890123".to_string(),
        amount: Some(50.0),
    };
    let result = any_id(config).unwrap();
    assert!(result.contains("1234567890123"));
    assert!(result.contains("540550.00"));
}

#[test]
fn test_bill_payment_with_amount() {
    let config = BillPaymentConfig {
        biller_id: "1234567890123".to_string(),
        amount: Some(100.50),
        ref1: "REF001".to_string(),
        ref2: None,
        ref3: None,
    };
    let result = bill_payment(config).unwrap();
    assert!(result.contains("5406100.50"));
    assert!(result.contains("REF001"));
}

#[test]
fn test_parse_with_sub_tags() {
    let payload = "00020101021229370016A0000006770101110113006680111111153037645802TH63041DCF";
    let result = parse(payload, false, true).unwrap();

    // Should be able to access sub-tags
    assert_eq!(
        result.get_tag_value("29", Some("00")).unwrap(),
        "A000000677010111"
    );
    assert_eq!(
        result.get_tag_value("29", Some("01")).unwrap(),
        "0066801111111"
    );
}

#[test]
fn test_parse_without_sub_tags() {
    let payload = "00020101021229370016A0000006770101110113006680111111153037645802TH63041DCF";
    let result = parse(payload, false, false).unwrap();

    // Should still be able to access main tags
    assert_eq!(result.get_tag_value("00", None).unwrap(), "01");
    assert_eq!(result.get_tag_value("01", None).unwrap(), "12"); // This should be "12" when amount is present

    // But sub-tag access should return None since sub_tags parsing was disabled
    // The tag 29 value should be the raw encoded string
    assert!(result.get_tag_value("29", None).is_some());
}

#[test]
fn test_true_money_with_message_encoding() {
    let config = TrueMoneyConfig {
        mobile_no: "0801111111".to_string(),
        amount: None,
        message: Some("Test".to_string()),
    };
    let result = true_money(config);
    assert!(result.contains("81160054006500730074"));
}

#[test]
fn test_bot_barcode_round_trip() {
    // Test that we can generate a BOT barcode and then parse it back
    let config = BotBarcodeConfig {
        biller_id: "123456789012345".to_string(),
        ref1: "REF123".to_string(),
        ref2: Some("REF456".to_string()),
        amount: Some(99.99),
    };

    let barcode_str = bot_barcode(config);
    let parsed_barcode = parse_barcode(&barcode_str).unwrap();

    assert_eq!(parsed_barcode.biller_id, "123456789012345");
    assert_eq!(parsed_barcode.ref1, "REF123");
    assert_eq!(parsed_barcode.ref2, Some("REF456".to_string()));
    assert_eq!(parsed_barcode.amount, Some(99.99));
}

#[test]
fn test_checksum_validation() {
    // Test that checksums are properly validated
    let valid_payload =
        "00020101021129370016A0000006770101110113006681222333353037645802TH63041DCF";
    assert!(parse(valid_payload, true, true).is_some());

    // Modify the checksum to make it invalid
    let invalid_payload =
        "00020101021129370016A0000006770101110113006681222333353037645802TH6304FFFF";
    assert!(parse(invalid_payload, true, true).is_none());
}
