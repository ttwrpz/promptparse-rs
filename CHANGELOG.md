# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2025-09-08

### Documentation
- Updated README keywords to include `"truemoney"` and remove `"qr"` for better search relevance

## [1.0.0] - 2025-09-07

### Added
- Initial Rust port of [maythiwat/promptparse](https://github.com/maythiwat/promptparse) v1.4.0
- Complete feature parity with TypeScript implementation
- **Parse** - PromptPay & EMVCo QR Code data strings into structs
- **Generate** - QR Code data from templates:
    - PromptPay AnyID (mobile number, National ID, Tax ID)
    - PromptPay Bill Payment with references
    - TrueMoney Wallet QR codes with messages
    - Slip Verify QR codes (Mini-QR for transaction verification)
    - BOT Barcode generation and conversion
- **Validate** - Checksum validation and data structure verification
- **Convert** - BOT Barcode to PromptPay QR Tag 30 conversion
- Comprehensive error handling with `PromptParseError`
- Full test suite ported from original implementation
- Extensive documentation and examples

### Technical Details
- Zero external dependencies
- Memory-safe implementation with Rust's ownership model
- Type-safe API with compile-time guarantees
- Cross-platform compatibility
- Performance optimizations over original implementation

### Documentation
- Complete README with usage examples
- Inline API documentation
- Working examples in `examples/` directory
- Proper attribution to original TypeScript library

## Original TypeScript Implementation

This Rust port is based on [promptparse](https://github.com/maythiwat/promptparse) by Maythiwat Chomchuen.

### Original Features Ported
- All QR code parsing and generation functionality
- All validation and verification features
- All test cases and expected behaviors
- Complete API compatibility (adapted for Rust idioms)

### Rust-Specific Improvements
- Compile-time type safety
- Memory safety guarantees
- Better error handling with `Result<T, E>`
- Zero-cost abstractions
- No runtime dependencies