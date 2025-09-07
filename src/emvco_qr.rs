use crate::tlv::{encode, get_tag, with_crc_tag, TlvTag};

#[derive(Debug, Clone)]
pub struct EmvCoQr {
    payload: String,
    tags: Vec<TlvTag>,
}

impl EmvCoQr {
    pub fn new(payload: String, tags: Vec<TlvTag>) -> Self {
        Self { payload, tags }
    }

    pub fn get_tag(&self, tag_id: &str, sub_tag_id: Option<&str>) -> Option<&TlvTag> {
        get_tag(&self.tags, tag_id, sub_tag_id)
    }

    pub fn get_tag_value(&self, tag_id: &str, sub_tag_id: Option<&str>) -> Option<&str> {
        self.get_tag(tag_id, sub_tag_id)
            .map(|tag| tag.value.as_str())
    }

    pub fn get_tags(&self) -> &[TlvTag] {
        &self.tags
    }

    pub fn get_payload(&self) -> &str {
        &self.payload
    }

    pub fn validate(&self, crc_tag_id: &str) -> bool {
        let tags: Vec<TlvTag> = self
            .tags
            .iter()
            .filter(|tag| tag.id != crc_tag_id)
            .cloned()
            .collect();

        let expected = with_crc_tag(&encode(&tags), crc_tag_id, true);
        self.payload == expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tlv::tag;

    #[test]
    fn test_emvco_qr_creation() {
        let tags = vec![tag("00", "01"), tag("01", "11")];
        let payload = "000201010111".to_string();
        let qr = EmvCoQr::new(payload.clone(), tags);

        assert_eq!(qr.get_payload(), &payload);
        assert_eq!(qr.get_tags().len(), 2);
        assert_eq!(qr.get_tag_value("00", None), Some("01"));
        assert_eq!(qr.get_tag_value("01", None), Some("11"));
    }

    #[test]
    fn test_emvco_qr_validation() {
        let tags = vec![
            tag("00", "01"),
            tag("01", "11"),
            tag("63", "1234"), // This would be the CRC tag
        ];
        let payload = "0002010101116304ABCD".to_string(); // Example payload
        let qr = EmvCoQr::new(payload, tags);

        // Test validation (need a proper CRC to pass)
        let result = qr.validate("63");
        assert!(!result, "Expected invalid CRC to return false");
    }
}
