use chrono::Duration;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct XmlIntValue {
    pub value: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlSignedValue {
    pub value: i64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlFloatValue {
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlStringValue {
    pub value: String,
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
