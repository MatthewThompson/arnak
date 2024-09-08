use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::utils::utc_date_time_deserializer;

/// A struct with information for a guild, returned by the guild endpoint of the API.
/// If requested it can also return the guild members, but only up to 25 at a time.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Guild {
    /// The ID of the guild.
    pub id: u64,
    /// The name of the guild.
    pub name: String,
    /// The date and time the guild was created.
    #[serde(rename = "created", with = "utc_date_time_deserializer")]
    pub created_at: DateTime<Utc>,
    /// Category of the guild such as event.
    pub category: String,
    /// Website for the guild.
    pub website: String,
    /// Boardgamegeek username of the manager of this guild.
    pub manager: String,
    /// Description of what the guild is for.
    pub description: String,
    /// The location of the guild. Some or all of the location values
    /// may be empty.
    pub location: Location,
    /// A page of the guild's members, up to a maximum of 25. Will be omitted
    /// if not requested, or if the page requested is out of bounds for the
    /// number of members.
    #[serde(rename = "members")]
    pub member_page: Option<MemberPage>,
}

/// A page of members in a particular guild, up to 25.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MemberPage {
    /// The total number of members in the guild. Not the number in this
    /// page.
    #[serde(rename = "count")]
    pub total_members: u64,
    /// The index of this page, starting from 1.
    #[serde(rename = "page")]
    pub page_number: u64,
    /// A list of members in this guid.
    #[serde(rename = "member")]
    pub members: Vec<Member>,
}

/// A member of a guild.
///
/// Includes their boardgamegeek username, and the date they joined the guild.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Member {
    /// The username of the guild member.
    pub name: String,
    /// The date and time the user joined the guild, in Utc.
    #[serde(rename = "date", with = "utc_date_time_deserializer")]
    pub date_joined: DateTime<Utc>,
}

/// A location of a guild. It is optional to set so some or
/// all values may be empty strings.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Location {
    /// First line of the guild address.
    #[serde(rename = "addr1")]
    pub address_line_1: String,
    /// Second line of the guild address.
    #[serde(rename = "addr2")]
    pub address_line_2: String,
    /// City where the guild is based.
    pub city: String,
    /// State/province/county where the guild is based.
    #[serde(rename = "stateorprovince")]
    pub state: String,
    /// Country where the guild is based.
    pub country: String,
    /// Guild address postal code.
    #[serde(rename = "postalcode")]
    pub postal_code: String,
}
