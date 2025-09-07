use promptparse::{
    generate::{any_id, bill_payment, bot_barcode, slip_verify, true_money},
    generate::{
        AnyIdConfig, BillPaymentConfig, BotBarcodeConfig, ProxyType, SlipVerifyConfig,
        TrueMoneyConfig,
    },
    parse, parse_barcode, validate,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("PromptParse Rust Library Examples\n");

    // Example 1: Parse QR Code
    println!("1. Parsing QR Code:");
    let qr_payload =
        "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
    if let Some(qr) = parse(qr_payload, true, true) {
        println!("   Parsed successfully!");
        println!("   Tag count: {}", qr.get_tags().len());
        if let Some(amount) = qr.get_tag_value("54", None) {
            println!("   Amount: {amount}");
        }
    }

    // Example 2: Generate PromptPay AnyID QR
    println!("\n2. Generating PromptPay AnyID QR:");
    let any_id_config = AnyIdConfig {
        proxy_type: ProxyType::Msisdn,
        target: "0812223333".to_string(),
        amount: Some(100.50),
    };
    let any_id_qr = any_id(any_id_config)?;
    println!("   Generated QR: {any_id_qr}");

    // Example 3: Generate Bill Payment QR
    println!("\n3. Generating Bill Payment QR:");
    let bill_config = BillPaymentConfig {
        biller_id: "1234567890123".to_string(),
        amount: Some(500.0),
        ref1: "CUSTOMER001".to_string(),
        ref2: Some("INV001".to_string()),
        ref3: None,
    };
    let bill_qr = bill_payment(bill_config)?;
    println!("   Generated QR: {bill_qr}");

    // Example 4: Generate TrueMoney QR
    println!("\n4. Generating TrueMoney QR:");
    let true_money_config = TrueMoneyConfig {
        mobile_no: "0801111111".to_string(),
        amount: Some(25.0),
        message: Some("Coffee money".to_string()),
    };
    let true_money_qr = true_money(true_money_config);
    println!("   Generated QR: {true_money_qr}");

    // Example 5: Generate Slip Verify QR
    println!("\n5. Generating Slip Verify QR:");
    let slip_config = SlipVerifyConfig {
        sending_bank: "014".to_string(),
        trans_ref: "REF123456789".to_string(),
    };
    let slip_qr = slip_verify(slip_config);
    println!("   Generated QR: {slip_qr}");

    // Example 6: Generate BOT Barcode
    println!("\n6. Generating BOT Barcode:");
    let bot_config = BotBarcodeConfig {
        biller_id: "099999999999990".to_string(),
        ref1: "111222333444".to_string(),
        ref2: Some("REF2".to_string()),
        amount: Some(150.0),
    };
    let bot_code = bot_barcode(bot_config);
    println!("   Generated Barcode: {bot_code}");

    // Example 7: Parse and convert BOT Barcode
    println!("\n7. Parsing and converting BOT Barcode:");
    let barcode_payload = "|099999999999990\r111222333444\r\r0";
    if let Some(barcode) = parse_barcode(barcode_payload) {
        println!("   Parsed barcode successfully!");
        println!("   Biller ID: {}", barcode.biller_id);
        println!("   Ref1: {}", barcode.ref1);

        if let Ok(qr_equivalent) = barcode.to_qr_tag30() {
            println!("   Converted to QR: {qr_equivalent}");
        }
    }

    // Example 8: Validate Slip Verify QR
    println!("\n8. Validating Slip Verify QR:");
    let slip_payload = "004100060000010103014022000111222233344ABCD125102TH910417DF";
    if let Some(slip_data) = validate::slip_verify(slip_payload) {
        println!("   Valid slip verify QR!");
        println!("   Bank: {}", slip_data.sending_bank);
        println!("   Transaction Ref: {}", slip_data.trans_ref);
    }

    println!("\nAll examples completed successfully!");
    Ok(())
}
