use core::fmt;

use chrono::Duration;
use serde::Deserialize;

use crate::{
    CollectionItemRating, CollectionItemRatingBrief, HotItem, RankValue, Ranks, WishlistPriority,
};

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

pub(crate) fn deserialize_rank_value_enum<'de, D>(deserializer: D) -> Result<RankValue, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    if s == "Not Ranked" {
        return Ok(RankValue::NotRanked);
    }

    let rank: Result<u64, _> = s.parse();
    match rank {
        Ok(value) => Ok(RankValue::Ranked(value)),
        _ => Err(serde::de::Error::unknown_variant(
            &s,
            &["u64", "Not Ranked"],
        )),
    }
}

impl<'de> Deserialize<'de> for HotItem {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            ID,
            Rank,
            Thumbnail,
            Name,
            YearPublished,
        }

        struct HotItemVisitor;

        impl<'de> serde::de::Visitor<'de> for HotItemVisitor {
            type Value = HotItem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML an item off the hot list.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut rank = None;
                let mut thumbnail = None;
                let mut name = None;
                let mut year_published = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ID => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            let id_str: String = map.next_value()?;
                            id = Some(id_str.parse::<u64>().map_err(|e| {
                                serde::de::Error::custom(format!(
                                    "failed to parse value a u64: {e}"
                                ))
                            })?);
                        }
                        Field::Rank => {
                            if rank.is_some() {
                                return Err(serde::de::Error::duplicate_field("rank"));
                            }
                            let rank_str: String = map.next_value()?;
                            rank = Some(rank_str.parse::<u64>().map_err(|e| {
                                serde::de::Error::custom(format!(
                                    "failed to parse value a u64: {e}"
                                ))
                            })?);
                        }
                        Field::Thumbnail => {
                            if thumbnail.is_some() {
                                return Err(serde::de::Error::duplicate_field("thumbnail"));
                            }
                            let thumbnail_xml_tag: XmlStringValue = map.next_value()?;
                            thumbnail = Some(thumbnail_xml_tag.value);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            let name_xml_tag: XmlStringValue = map.next_value()?;
                            name = Some(name_xml_tag.value);
                        }
                        Field::YearPublished => {
                            if year_published.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearpublished"));
                            }
                            let year_published_xml_tag: XmlSignedValue = map.next_value()?;
                            year_published = Some(year_published_xml_tag.value);
                        }
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let rank = rank.ok_or_else(|| serde::de::Error::missing_field("rank"))?;
                let thumbnail =
                    thumbnail.ok_or_else(|| serde::de::Error::missing_field("thumbnail"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let year_published = year_published
                    .ok_or_else(|| serde::de::Error::missing_field("yearpublished"))?;
                Ok(Self::Value {
                    id,
                    rank,
                    thumbnail,
                    name,
                    year_published,
                })
            }
        }
        deserializer.deserialize_any(HotItemVisitor)
    }
}

pub(crate) fn deserialize_game_ratings<'de, D>(
    deserializer: D,
) -> Result<CollectionItemRating, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
        Value,
        UsersRated,
        Average,
        Bayesaverage,
        Stddev,
        Median,
        Ranks,
    }

    struct CollectionItemRatingVisitor;

    impl<'de> serde::de::Visitor<'de> for CollectionItemRatingVisitor {
        type Value = CollectionItemRating;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing the XML for a user's rating of a board game, which includes the average rating on the site and the number of ratings.")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut user_rating = None;
            let mut users_rated = None;
            let mut average = None;
            let mut bayesian_average = None;
            let mut standard_deviation = None;
            let mut median = None;
            let mut ranks = None;
            while let Some(key) = map.next_key()? {
                match key {
                    Field::Value => {
                        if user_rating.is_some() {
                            return Err(serde::de::Error::duplicate_field("value"));
                        }
                        let user_rating_str: String = map.next_value()?;
                        user_rating = match user_rating_str.as_str() {
                            "N/A" => Some(None),
                            other => Some(Some(other.parse::<f64>().map_err(|e| {
                                serde::de::Error::custom(format!(
                                    "failed to parse value as N/A or float: {e}"
                                ))
                            })?)),
                        }
                    }
                    Field::UsersRated => {
                        if users_rated.is_some() {
                            return Err(serde::de::Error::duplicate_field("usersrated"));
                        }
                        let users_rated_xml_tag: XmlIntValue = map.next_value()?;
                        users_rated = Some(users_rated_xml_tag.value);
                    }
                    Field::Average => {
                        if average.is_some() {
                            return Err(serde::de::Error::duplicate_field("average"));
                        }
                        let average_xml_tag: XmlFloatValue = map.next_value()?;
                        average = Some(average_xml_tag.value);
                    }
                    Field::Bayesaverage => {
                        if bayesian_average.is_some() {
                            return Err(serde::de::Error::duplicate_field("bayesaverage"));
                        }
                        let bayesian_average_xml_tag: XmlFloatValue = map.next_value()?;
                        bayesian_average = Some(bayesian_average_xml_tag.value);
                    }
                    Field::Stddev => {
                        if standard_deviation.is_some() {
                            return Err(serde::de::Error::duplicate_field("stddev"));
                        }
                        let standard_deviation_xml_tag: XmlFloatValue = map.next_value()?;
                        standard_deviation = Some(standard_deviation_xml_tag.value);
                    }
                    Field::Median => {
                        if median.is_some() {
                            return Err(serde::de::Error::duplicate_field("median"));
                        }
                        let median_xml_tag: XmlFloatValue = map.next_value()?;
                        median = Some(median_xml_tag.value);
                    }
                    Field::Ranks => {
                        if ranks.is_some() {
                            return Err(serde::de::Error::duplicate_field("ranks"));
                        }
                        // An extra layer of indirection is needed due to the way the XML is structured,
                        // but should be removed for the final structure.
                        let ranks_struct: Ranks = map.next_value()?;
                        ranks = Some(ranks_struct.ranks);
                    }
                }
            }
            let user_rating =
                user_rating.ok_or_else(|| serde::de::Error::missing_field("value"))?;
            let users_rated =
                users_rated.ok_or_else(|| serde::de::Error::missing_field("usersrated"))?;
            let average = average.ok_or_else(|| serde::de::Error::missing_field("average"))?;
            let bayesian_average =
                bayesian_average.ok_or_else(|| serde::de::Error::missing_field("bayesaverage"))?;
            let standard_deviation =
                standard_deviation.ok_or_else(|| serde::de::Error::missing_field("stddev"))?;
            let median = median.ok_or_else(|| serde::de::Error::missing_field("median"))?;
            let ranks = ranks.ok_or_else(|| serde::de::Error::missing_field("ranks"))?;
            Ok(Self::Value {
                user_rating,
                users_rated,
                average,
                bayesian_average,
                standard_deviation,
                median,
                ranks,
            })
        }
    }
    deserializer.deserialize_any(CollectionItemRatingVisitor)
}

pub(crate) fn deserialize_game_ratings_brief<'de, D>(
    deserializer: D,
) -> Result<CollectionItemRatingBrief, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(field_identifier, rename_all = "lowercase")]
    enum Field {
        Value,
        Average,
        Bayesaverage,
    }

    struct CollectionItemRatingBriefVisitor;

    impl<'de> serde::de::Visitor<'de> for CollectionItemRatingBriefVisitor {
        type Value = CollectionItemRatingBrief;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing the XML for a user's rating of a board game, which includes the average rating on the site and the number of ratings.")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut user_rating = None;
            let mut average = None;
            let mut bayesian_average = None;
            while let Some(key) = map.next_key()? {
                match key {
                    Field::Value => {
                        if user_rating.is_some() {
                            return Err(serde::de::Error::duplicate_field("value"));
                        }
                        let user_rating_str: String = map.next_value()?;
                        user_rating = match user_rating_str.as_str() {
                            "N/A" => Some(None),
                            other => Some(Some(other.parse::<f64>().map_err(|e| {
                                serde::de::Error::custom(format!(
                                    "failed to parse value as N/A or float: {e}"
                                ))
                            })?)),
                        }
                    }
                    Field::Average => {
                        if average.is_some() {
                            return Err(serde::de::Error::duplicate_field("average"));
                        }
                        let average_xml_tag: XmlFloatValue = map.next_value()?;
                        average = Some(average_xml_tag.value);
                    }
                    Field::Bayesaverage => {
                        if bayesian_average.is_some() {
                            return Err(serde::de::Error::duplicate_field("bayesaverage"));
                        }
                        let bayesian_average_xml_tag: XmlFloatValue = map.next_value()?;
                        bayesian_average = Some(bayesian_average_xml_tag.value);
                    }
                }
            }
            let user_rating =
                user_rating.ok_or_else(|| serde::de::Error::missing_field("value"))?;
            let average = average.ok_or_else(|| serde::de::Error::missing_field("average"))?;
            let bayesian_average =
                bayesian_average.ok_or_else(|| serde::de::Error::missing_field("bayesaverage"))?;
            Ok(Self::Value {
                user_rating,
                average,
                bayesian_average,
            })
        }
    }
    deserializer.deserialize_any(CollectionItemRatingBriefVisitor)
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
        // TODO check if this should be an error
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
