// Benchmarks for PromptParse operations
// Run with: cargo bench

use promptparse::{
    generate::{any_id, bill_payment, slip_verify, true_money},
    generate::{AnyIdConfig, BillPaymentConfig, ProxyType, SlipVerifyConfig, TrueMoneyConfig},
    parse, parse_barcode,
    tlv::{checksum, decode, encode, tag, with_crc_tag},
    validate,
};

macro_rules! bench {
    ($name:ident, $code:block) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let start = std::time::Instant::now();
            let iterations = 10000;

            for _ in 0..iterations {
                $code
            }

            let duration = start.elapsed();
            println!(
                "{}: {:?} per iteration",
                stringify!($name),
                duration / iterations
            );
        }
    };
}

bench!(bench_parse_qr, {
    let payload =
        "00020101021229370016A0000006770101110113006680111111153037645802TH540520.15630442BE";
    let _result = parse(payload, true, true);
});

bench!(bench_generate_any_id, {
    let config = AnyIdConfig {
        proxy_type: ProxyType::Msisdn,
        target: "0812223333".to_string(),
        amount: Some(100.0),
    };
    let _result = any_id(config);
});

bench!(bench_generate_bill_payment, {
    let config = BillPaymentConfig {
        biller_id: "0112233445566".to_string(),
        amount: Some(500.0),
        ref1: "CUSTOMER001".to_string(),
        ref2: Some("INV001".to_string()),
        ref3: None,
    };
    let _result = bill_payment(config);
});

bench!(bench_generate_true_money, {
    let config = TrueMoneyConfig {
        mobile_no: "0801111111".to_string(),
        amount: Some(25.0),
        message: Some("Hello World!".to_string()),
    };
    let _result = true_money(config);
});

bench!(bench_tlv_encode_decode, {
    let tags = vec![
        tag("00", "01"),
        tag("01", "11"),
        tag("29", "0016A0000006770101110113006681222333"),
        tag("53", "764"),
        tag("58", "TH"),
    ];
    let encoded = encode(&tags);
    let _decoded = decode(&encoded);
});

bench!(bench_checksum_calculation, {
    let payload = "00020101021229370016A0000006770101110113006681222333353037645802TH";
    let _checksum = checksum(payload, true);
});

bench!(bench_parse_barcode, {
    let payload = "|099999999999990\r111222333444\r\r0";
    let _result = parse_barcode(payload);
});

bench!(bench_validate_slip_verify, {
    let payload = "004100060000010103014022000111222233344ABCD125102TH910417DF";
    let _result = validate::slip_verify(payload);
});

bench!(bench_with_crc_tag, {
    let payload = "00020101021229370016A0000006770101110113006681222333353037645802TH";
    let _result = with_crc_tag(payload, "63", true);
});

bench!(bench_slip_verify_generation, {
    let config = SlipVerifyConfig {
        sending_bank: "014".to_string(),
        trans_ref: "REF123456789ABCDEF".to_string(),
    };
    let _result = slip_verify(config);
});

// Composite benchmark - full QR generation and parsing cycle
bench!(bench_full_cycle, {
    // Generate
    let config = AnyIdConfig {
        proxy_type: ProxyType::Msisdn,
        target: "0812223333".to_string(),
        amount: Some(100.0),
    };
    let qr_code = any_id(config).unwrap();

    // Parse
    let parsed = parse(&qr_code, true, true).unwrap();

    // Validate
    let _is_valid = parsed.validate("63");

    // Extract data
    let _amount = parsed.get_tag_value("54", None);
    let _mobile = parsed.get_tag_value("29", Some("01"));
});

#[cfg(test)]
mod benchmark_tests {
    use super::*;

    #[test]
    fn run_all_benchmarks() {
        println!("Running PromptParse Rust benchmarks...\n");

        bench_parse_qr();
        bench_generate_any_id();
        bench_generate_bill_payment();
        bench_generate_true_money();
        bench_tlv_encode_decode();
        bench_checksum_calculation();
        bench_parse_barcode();
        bench_validate_slip_verify();
        bench_with_crc_tag();
        bench_slip_verify_generation();
        bench_full_cycle();

        println!("\nBenchmarks completed!");
    }
}
