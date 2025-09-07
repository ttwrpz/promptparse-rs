/// Generate a UCS-2-like Hex string for Tag 81
///
/// This method is equivalent to:
/// `Buffer.from(message, 'utf16le').swap16().toString('hex').toUpperCase()`
///
/// # Arguments
/// * `message` - Message to encode
///
/// # Returns
/// Hex string of provided message
pub fn encode_tag81(message: &str) -> String {
    message
        .chars()
        .map(|c| format!("{:04X}", c as u32))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_tag81() {
        let message = "Hello World!";
        let result = encode_tag81(message);
        assert_eq!(result, "00480065006C006C006F00200057006F0072006C00640021");
        // This is 48 (H), 65 (e), 6C (l), 6C (l), 6F (o), 20 (space), etc.
    }

    #[test]
    fn test_encode_tag81_empty() {
        let result = encode_tag81("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_encode_tag81_special_chars() {
        let message = "åäö";
        let result = encode_tag81(message);
        assert_eq!(result, "00E500E400F6");
    }
}
