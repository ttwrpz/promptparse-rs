# Contributing to PromptParse (Rust)

Thank you for your interest in contributing to the Rust port of PromptParse! This document provides guidelines for contributing to the project.

## About This Project

This is a Rust port of the original [promptparse](https://github.com/maythiwat/promptparse) TypeScript library by [Maythiwat Chomchuen](https://github.com/maythiwat). All credit for the original design and implementation goes to the original author.

## Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Setting Up the Development Environment

1. Clone the repository:
   ```bash
   git clone https://github.com/ttwrpz/promptparse-rs.git
   cd promptparse-rs
   ```

2. Run the tests:
   ```bash
   cargo test
   ```

3. Run the examples:
   ```bash
   cargo run --example basic_usage
   ```

## Development Guidelines

### Code Style

- Follow standard Rust formatting with `rustfmt`:
  ```bash
  cargo fmt
  ```

- Use `clippy` for linting:
  ```bash
  cargo clippy
  ```

- Ensure all tests pass:
  ```bash
  cargo test
  ```

### Testing

- All new features should include comprehensive tests
- Tests should match the behavior of the original TypeScript implementation
- Test names should be descriptive and follow the pattern `test_feature_scenario`
- Include both positive and negative test cases

### Documentation

- All public APIs must be documented with doc comments
- Include examples in doc comments where appropriate
- Update the README if adding new features
- Keep the CHANGELOG.md updated

## Contributing Process

### 1. Issues

- Check existing issues before creating a new one
- Use the issue templates when available
- Provide clear reproduction steps for bugs
- Include relevant error messages and stack traces

### 2. Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes following the development guidelines
4. Add tests for your changes
5. Update documentation as needed
6. Ensure all tests pass: `cargo test`
7. Run formatting and linting: `cargo fmt && cargo clippy`
8. Commit your changes with clear messages
9. Push to your fork and create a pull request

### 3. Commit Messages

Use clear, descriptive commit messages:
- `feat: add support for new QR code type`
- `fix: correct checksum calculation for edge case`
- `docs: update README with new examples`
- `test: add tests for bill payment validation`

## Types of Contributions

### Bug Fixes
- Fix parsing errors or incorrect QR code generation
- Correct documentation mistakes
- Improve error messages

### Features
- **Important**: New features should ideally match functionality in the original TypeScript library
- If adding Rust-specific features, clearly document the differences
- Ensure backward compatibility when possible

### Documentation
- Improve existing documentation
- Add more examples
- Translate documentation to other languages

### Performance
- Optimize existing algorithms
- Reduce memory usage
- Improve compilation times

## Maintaining Compatibility

### With Original Library
- Maintain feature parity with the original TypeScript implementation
- Ensure test results match expected behaviors from the original
- Document any intentional differences (e.g., Rust-specific error handling)

### API Stability
- Avoid breaking changes to public APIs
- Use deprecation warnings when removing features
- Follow semantic versioning

## Reference Materials

When contributing, refer to these resources:

- [Original TypeScript Implementation](https://github.com/maythiwat/promptparse)
- [EMV QR Code Specification](https://www.emvco.com/emv-technologies/qrcodes/)
- [Thai QR Payment Standard](https://www.bot.or.th/content/dam/bot/fipcs/documents/FPG/2562/ThaiPDF/25620084.pdf)
- [BOT Barcode Standard](https://www.bot.or.th/content/dam/bot/documents/th/our-roles/payment-systems/about-payment-systems/Std_Barcode.pdf)

## License

By contributing to this project, you agree that your contributions will be licensed under the same MIT License that covers the project. See [LICENSE](LICENSE) for details.

## Questions?

If you have questions about contributing, feel free to:
- Open an issue for discussion
- Reference the original TypeScript implementation for expected behavior
- Check the existing tests for examples

Thank you for helping improve PromptParse for the Rust community! 🦀