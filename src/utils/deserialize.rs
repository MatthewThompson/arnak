use chrono::Duration;
use serde::Deserialize;

use crate::{ItemType, NameType};

pub(crate) fn deserialise_xml_string<T: serde::de::DeserializeOwned>(
    xml: &str,
) -> core::result::Result<T, serde_xml_rs::Error> {
    // The parser config used by serde_xml
    let default_xml_reader_config = xml::ParserConfig::new()
        .trim_whitespace(true)
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .ignore_comments(true)
        .coalesce_characters(true);
    // Not allowed by the default XML spec, so the underlying XML reader will return an error
    // while trying to deserialise. But this is used by boardgamegeek in the descriptions so
    // we need to add it here.
    let xml_reader_config = default_xml_reader_config.add_entity("mdash", "—");

    let xml_reader = xml::reader::EventReader::new_with_config(xml.as_bytes(), xml_reader_config);
    let mut deserializer = serde_xml_rs::Deserializer::new(xml_reader);
    T::deserialize(&mut deserializer)
}

// Types that only exist as intermediary values when deserialising more complex types.
// They appear in the form `<tag value="some_value">`

#[derive(Debug, Deserialize)]
pub(crate) struct XmlIntValue {
    pub(crate) value: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlSignedValue {
    pub(crate) value: i64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlFloatValue {
    pub(crate) value: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlStringValue {
    pub(crate) value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlName {
    #[serde(rename = "type")]
    pub(crate) name_type: NameType,
    pub(crate) value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlLink {
    #[serde(rename = "type")]
    pub(crate) link_type: ItemType,
    pub(crate) id: u64,
    pub(crate) value: String,
}

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

pub(crate) fn deserialize_minutes<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    // Parse as unsigned because although Duration supports negative durations,
    // we do not want to support that for game playing time.
    let minutes = s.parse::<u32>().map_err(|e| {
        serde::de::Error::custom(format!(
            "unable to parse duration minutes string to u32: {e}"
        ))
    });
    minutes.map(|m| Duration::minutes(m as i64))
}

pub(crate) mod date_deserializer {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

pub(crate) mod utc_date_time_deserializer {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // e.g. Thu, 14 Jun 2007 01:06:46 +0000
        let dt = NaiveDateTime::parse_from_str(&s, "%a, %d %B %Y %H:%M:%S %z")
            .map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
