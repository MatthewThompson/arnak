pub(crate) fn deserialize_1_0_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(&s, &["1", "0"])),
    }
}