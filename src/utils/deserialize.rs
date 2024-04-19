use crate::WishlistPriority;

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

pub(crate) fn deserialize_wishlist_priority<'de, D>(
    deserializer: D,
) -> Result<Option<WishlistPriority>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "5" => Ok(Some(WishlistPriority::DontBuyThis)),
        "4" => Ok(Some(WishlistPriority::ThinkingAboutIt)),
        "3" => Ok(Some(WishlistPriority::LikeToHave)),
        "2" => Ok(Some(WishlistPriority::LoveToHave)),
        "1" => Ok(Some(WishlistPriority::MustHave)),
        _ => Ok(None),
    }
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
