use std::borrow::Cow;

// Function taken from quick-xml: https://docs.rs/quick-xml/latest/src/quick_xml/escapei.rs
// but changed to only care about ampersands.
pub(crate) fn escape_xml(xml_str: &str) -> Cow<str> {
    let bytes = xml_str.as_bytes();
    let mut escaped = None;
    let mut iter = bytes.iter();
    let mut pos = 0;
    while let Some(i) = iter.position(|&b| b == b'&') {
        if escaped.is_none() {
            escaped = Some(Vec::with_capacity(xml_str.len()));
        }
        let escaped = escaped.as_mut().unwrap();
        let new_pos = pos + i;
        escaped.extend_from_slice(&bytes[pos..new_pos]);
        escaped.extend_from_slice(b"&amp;");
        pos = new_pos + 1;
    }

    if let Some(mut escaped) = escaped {
        if let Some(raw) = bytes.get(pos..) {
            escaped.extend_from_slice(raw);
        }
        // SAFETY: we operate on UTF-8 input and search for one byte chars only,
        // so all slices that was put to the `escaped` is a valid UTF-8 encoded strings
        Cow::Owned(String::from_utf8(escaped).unwrap())
    } else {
        Cow::Borrowed(xml_str)
    }
}
