# PromptParse (Rust)

[![Crates.io](https://img.shields.io/crates/v/promptparse.svg)](https://crates.io/crates/promptparse)
[![Documentation](https://docs.rs/promptparse/badge.svg)](https://docs.rs/promptparse)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

"All-in-one Rust library for PromptPay & EMVCo QR Codes"

No external dependencies & Cross-platform. You can use it anywhere!

This is a Rust port of [maythiwat/promptparse](https://github.com/maythiwat/promptparse)

## Features

- **Parse** — PromptPay & EMVCo QR Code data strings into structs
- **Generate** — QR Code data from pre-made templates (PromptPay AnyID, PromptPay Bill Payment, TrueMoney, etc.)
- **Manipulate** — any values from parsed QR Code data and encode back into QR Code data
- **Validate** — checksum and data structure for known QR Code formats (Slip Verify API Mini QR)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
promptparse = "1.0.0"
```

## Usage

### Parsing data and get value from tag

```rust
use promptparse::parse;

fn main() {
    // Example data
    let ppqr = parse("000201010211...", false, true).unwrap();

    // Get Value of Tag ID '00'
    let value = ppqr.get_tag_value("00", None).unwrap(); // Returns "01"
}
```

### Build QR data and append CRC tag

```rust
use promptparse::{encode, tag, with_crc_tag};

fn main() {
    // Example data
    let data = vec![
        tag("00", "01"),
        tag("01", "11"),
        // ...
    ];

    // Set CRC Tag ID '63'
    let result = with_crc_tag(&encode(&data), "63", true); // Returns "000201010211..."
}
```

### Generate PromptPay Bill Payment QR

```rust
use promptparse::generate::{bill_payment, BillPaymentConfig};

fn main() {
    let config = BillPaymentConfig {
        biller_id: "1xxxxxxxxxxxx".to_string(),
        amount: Some(300.0),
        ref1: "INV12345".to_string(),
        ref2: None,
        ref3: None,
    };

    let payload = bill_payment(config).unwrap();
    // TODO: Create QR Code from payload
}
```

### Generate PromptPay AnyID QR

```rust
use promptparse::generate::{any_id, AnyIdConfig, ProxyType};

fn main() {
    let config = AnyIdConfig {
        proxy_type: ProxyType::Msisdn,
        target: "0812223333".to_string(),
        amount: Some(100.0),
    };

    let payload = any_id(config).unwrap();
    // TODO: Create QR Code from payload
}
```

### Generate TrueMoney QR

```rust
use promptparse::generate::{true_money, TrueMoneyConfig};

fn main() {
    let config = TrueMoneyConfig {
        mobile_no: "08xxxxxxxx".to_string(),
        amount: Some(10.0),
        message: Some("Hello World!".to_string()),
    };

    let payload = true_money(config);
    // TODO: Create QR Code from payload
}
```

### Validate & extract data from Slip Verify QR

```rust
use promptparse::validate;

fn main() {
    let data = validate::slip_verify("00550006000001...").unwrap();
    println!("Sending Bank: {}", data.sending_bank);
    println!("Transaction Ref: {}", data.trans_ref);

    // TODO: Inquiry transaction from Bank Open API
}
```

### Convert BOT Barcode to PromptPay QR Tag 30 (Bill Payment)

```rust
use promptparse::parse_barcode;

fn main() {
    let bot_barcode = parse_barcode("|310109999999901\r...").unwrap();
    let payload = bot_barcode.to_qr_tag30().unwrap();

    // TODO: Create QR Code from payload
}
```

## Error Handling

The library uses `Result<T, PromptParseError>` for operations that can fail:

```rust
use promptparse::{parse, PromptParseError};

fn main() {
    match parse("invalid_data", true, true) {
        Some(qr) => {
            println!("Parsed successfully: {:?}", qr);
        }
        None => {
            println!("Failed to parse QR code");
        }
    }
}
```

## About This Port

This Rust implementation is a faithful port of the excellent [promptparse](https://github.com/maythiwat/promptparse) TypeScript/JavaScript library by [Maythiwat Chomchuen](https://github.com/maythiwat). The original library has been thoroughly tested and is widely used in the Thai payment ecosystem.

All credit for the original design, implementation, and testing goes to the original author. This port aims to bring the same reliability and functionality to Rust developers.

## References

- [EMV QR Code](https://www.emvco.com/emv-technologies/qrcodes/)
- [Thai QR Payment Standard](https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2562/ThaiPDF/25620084.pdf)
- [Slip Verify API Mini QR Data](https://developer.scb/assets/documents/documentation/qr-payment/extracting-data-from-mini-qr.pdf)
- [BOT Barcode Standard](https://www.bot.or.th/content/dam/bot/documents/th/our-roles/payment-systems/about-payment-systems/Std_Barcode.pdf)

## See Also

- [maythiwat/promptparse](https://github.com/maythiwat/promptparse) - Original TypeScript/JavaScript implementation
- [phoomin2012/promptparse-php](https://github.com/phoomin2012/promptparse-php) - PHP implementation

## License

This project is MIT licensed (see [LICENSE](LICENSE)), maintaining the same license as the original work.