use crate::error::PromptParseError;
use crate::utils::checksum::crc16_xmodem;

#[derive(Debug, Clone, PartialEq)]
pub struct TlvTag {
    /// Tag ID
    pub id: String,
    /// Tag Value
    pub value: String,
    /// Sub Tags
    pub sub_tags: Option<Vec<TlvTag>>,
    /// Tag Length
    pub length: usize,
}

impl TlvTag {
    pub fn new(id: String, value: String) -> Self {
        Self {
            length: value.len(),
            id,
            value,
            sub_tags: None,
        }
    }

    pub fn with_sub_tags(id: String, value: String, sub_tags: Vec<TlvTag>) -> Self {
        Self {
            length: value.len(),
            id,
            value,
            sub_tags: Some(sub_tags),
        }
    }
}

/// Decode TLV string into array of TLV Tags
pub fn decode(payload: &str) -> Result<Vec<TlvTag>, PromptParseError> {
    let mut tags = Vec::new();
    let mut idx = 0;

    while idx < payload.len() {
        if idx + 4 > payload.len() {
            break;
        }

        let id = payload[idx..idx + 2].to_string();
        let length_str = &payload[idx + 2..idx + 4];
        let length = length_str
            .parse::<usize>()
            .map_err(|_| PromptParseError::InvalidTlv)?;

        if idx + 4 + length > payload.len() {
            return Err(PromptParseError::InvalidTlv);
        }

        let value = payload[idx + 4..idx + 4 + length].to_string();

        tags.push(TlvTag::new(id, value));
        idx += 4 + length;
    }

    Ok(tags)
}

/// Encode TLV Tags array into TLV string
pub fn encode(tags: &[TlvTag]) -> String {
    let mut payload = String::new();

    for tag in tags {
        payload.push_str(&tag.id);
        payload.push_str(&format!("{:02}", tag.length));

        if let Some(sub_tags) = &tag.sub_tags {
            payload.push_str(&encode(sub_tags));
        } else {
            payload.push_str(&tag.value);
        }
    }

    payload
}

/// Generate CRC Checksum for provided string
pub fn checksum(payload: &str, upper_case: bool) -> String {
    let mut sum = format!("{:x}", crc16_xmodem(payload, 0xffff));
    if upper_case {
        sum = sum.to_uppercase();
    }
    format!("{sum:0>4}")
}

/// Get TLV string combined with CRC Tag
pub fn with_crc_tag(payload: &str, crc_tag_id: &str, upper_case: bool) -> String {
    let mut result = payload.to_string();
    result.push_str(&format!("{crc_tag_id:0>2}"));
    result.push_str("04");
    result.push_str(&checksum(&result, upper_case));
    result
}

/// Get Tag or Sub-tag by Tag ID in array of TLV Tags
pub fn get_tag<'a>(
    tlv_tags: &'a [TlvTag],
    tag_id: &str,
    sub_tag_id: Option<&str>,
) -> Option<&'a TlvTag> {
    let tag = tlv_tags.iter().find(|t| t.id == tag_id)?;

    if let Some(sub_id) = sub_tag_id {
        if let Some(sub_tags) = &tag.sub_tags {
            return sub_tags.iter().find(|s| s.id == sub_id);
        }
        return None;
    }

    Some(tag)
}

/// Create new TLV Tag
pub fn tag(tag_id: &str, value: &str) -> TlvTag {
    TlvTag::new(tag_id.to_string(), value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tlv_encode_decode() {
        let tags = vec![tag("00", "01"), tag("01", "11"), tag("02", "test")];

        let encoded = encode(&tags);
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0].id, "00");
        assert_eq!(decoded[0].value, "01");
        assert_eq!(decoded[1].id, "01");
        assert_eq!(decoded[1].value, "11");
        assert_eq!(decoded[2].id, "02");
        assert_eq!(decoded[2].value, "test");
    }

    #[test]
    fn test_checksum() {
        let payload =
            "00020101021229370016A0000006770101110113006680111111153037645802TH540520.156304";
        let result = checksum(payload, true);
        assert_eq!(result, "42BE");
    }

    #[test]
    fn test_with_crc_tag() {
        let payload = "00020101021129370016A0000006770101110113006681222333353037645802TH";
        let result = with_crc_tag(payload, "63", true);
        assert_eq!(
            result,
            "00020101021129370016A0000006770101110113006681222333353037645802TH63041DCF"
        );
    }

    #[test]
    fn test_get_tag() {
        let tags = vec![tag("00", "01"), tag("01", "11"), tag("02", "test")];

        let found = get_tag(&tags, "01", None).unwrap();
        assert_eq!(found.value, "11");

        let not_found = get_tag(&tags, "99", None);
        assert!(not_found.is_none());
    }
}
