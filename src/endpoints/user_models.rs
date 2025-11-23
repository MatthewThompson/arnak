use core::fmt;
use std::string::String;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::deserialize::{XmlDateValue, XmlIntValue, XmlSignedValue, XmlStringValue};

/// A user's information.
#[derive(Clone, Debug, PartialEq)]
pub struct User {
    /// The ID of the user.
    pub id: u64,
    /// The username of the user.
    pub username: String,
    /// The first name of the user.
    pub first_name: String,
    /// The last name of the user.
    pub last_name: String,
    /// A link to the user's avatar.
    pub avatar_link: Option<String>,
    /// The year that the user registered on `BoardGameGeek`.
    pub year_registered: i64,
    /// The date that the user last logged in.
    pub last_login: NaiveDate,
    /// State or province that the user lives in.
    pub state_or_province: Option<String>,
    /// Country the user lives in.
    pub country: Option<String>,
    /// User's website.
    pub web_address: Option<String>,
    /// User's XBOX gamertag.
    pub xbox_account: Option<String>,
    /// User's Wii account username.
    pub wii_account: Option<String>,
    /// User's PSN username.
    pub psn_account: Option<String>,
    /// User's battle.net username.
    pub battlenet_account: Option<String>,
    /// User's steam username.
    pub steam_account: Option<String>,
    /// A rating, starting a 0, gained by performing board game trades with other users.
    pub trade_rating: u64,
    /// The user's top 10 list.
    pub top_list: Vec<ListItem>,
    /// The user's hot 10 list.
    pub hot_list: Vec<ListItem>,
    /// A list of the users that this user is buddies with.
    pub buddies: BuddyList,
    /// A list of the guilds that this user belongs to.
    pub guilds: GuildList,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct ItemList {
    #[serde(default, rename = "item")]
    pub(crate) items: Vec<ListItem>,
}

/// An item in a user made top 10 or hot 10 list. A brief representation of an item which
/// contains a name type and rank.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ListItem {
    /// The ID of this item.
    pub id: u64,
    /// The name of the game or person. It can also be a name of a mechanic such as worker
    /// placement.
    pub name: String,
    /// A number 1 through 10, with 1 being top of the list.
    pub rank: u64,
    /// The type of item, which may be a game, person, event.
    #[serde(rename = "type")]
    pub item_type: ListItemType,
}

/// A type of item in a user's top 10, or hot 10 list on their profile.
/// Note that when choosing items on the website
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ListItemType {
    /// A board game, board game expansion, or board game accessory.
    Thing,
    /// A person who is a board game designer, publisher, or artist.
    Person,
    /// A company designers, or publishes, games.
    Company,
    /// A certain family of games, can be a certain category, a certain component, or theme. Can
    /// also be an accessory family.
    Family,
    /// A mechanic or category for a game. Mechanics include dice rolling, worker placement, and
    /// cooperative game. Categories include word game, and party game.
    Property,
    /// A board game event or convention.
    Event,
}

/// A single page of guilds the user belongs to, also includes the total number of guilds the
/// user belongs to.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct GuildList {
    /// The total number of guilds this user belongs to.
    pub total: u64,
    /// The page number for the guilds in the list.
    pub page: u64,
    /// The list of guilds.
    #[serde(default, rename = "guild")]
    pub guilds: Vec<GuildBrief>,
}

/// A guild a user belongs to, including only the name and ID.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct GuildBrief {
    /// The ID of the guild.
    pub id: u64,
    /// The name of the guild that the user belongs to.
    pub name: String,
}

/// A single page of buddies that the users has, also includes the total number of buddies
/// the user has.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct BuddyList {
    /// The total number of buddies this user has.
    pub total: u64,
    /// The page number for the buddies in the list.
    pub page: u64,
    /// The list of buddies.
    #[serde(default, rename = "buddy")]
    pub buddies: Vec<Buddy>,
}

/// A user that this user is buddies with.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Buddy {
    /// The ID of the user.
    pub id: u64,
    /// The user's username.
    pub name: String,
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            TermsOfUse,
            Id,
            Name,
            FirstName,
            LastName,
            AvatarLink,
            YearRegistered,
            LastLogin,
            StateOrProvince,
            Country,
            WebAddress,
            XboxAccount,
            WiiAccount,
            PsnAccount,
            BattlenetAccount,
            SteamAccount,
            TradeRating,
            Top,
            Hot,
            Guilds,
            Buddies,
        }

        struct UserVisitor;

        impl<'de> serde::de::Visitor<'de> for UserVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML for a user.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut username = None;
                let mut first_name = None;
                let mut last_name = None;
                let mut avatar_link = None;
                let mut year_registered = None;
                let mut last_login = None;
                let mut state_or_province = None;
                let mut country = None;
                let mut web_address = None;
                let mut xbox_account = None;
                let mut wii_account = None;
                let mut psn_account = None;
                let mut battlenet_account = None;
                let mut steam_account = None;
                let mut trade_rating = None;
                let mut top_list = None;
                let mut hot_list = None;
                let mut guild_list = None;
                let mut buddy_list = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::TermsOfUse => {
                            // Ignore
                            let _: String = map.next_value()?;
                        },
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        Field::Name => {
                            if username.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            username = Some(map.next_value()?);
                        },
                        Field::FirstName => {
                            if first_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstname"));
                            }
                            let first_name_xml: XmlStringValue = map.next_value()?;
                            first_name = Some(first_name_xml.value);
                        },
                        Field::LastName => {
                            if last_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastname"));
                            }
                            let last_name_xml: XmlStringValue = map.next_value()?;
                            last_name = Some(last_name_xml.value);
                        },
                        Field::AvatarLink => {
                            if avatar_link.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarlink"));
                            }
                            let avatar_link_xml: XmlStringValue = map.next_value()?;
                            avatar_link = if avatar_link_xml.value == "N/A" {
                                None
                            } else {
                                Some(avatar_link_xml.value)
                            }
                        },
                        Field::YearRegistered => {
                            if year_registered.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearregistered"));
                            }
                            let year_registered_xml: XmlSignedValue = map.next_value()?;
                            year_registered = Some(year_registered_xml.value);
                        },
                        Field::LastLogin => {
                            if last_login.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastlogin"));
                            }
                            let last_login_xml: XmlDateValue = map.next_value()?;
                            last_login = Some(last_login_xml.value);
                        },
                        Field::StateOrProvince => {
                            if state_or_province.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateorprovince"));
                            }
                            let state_or_province_xml: XmlStringValue = map.next_value()?;
                            state_or_province = Some(state_or_province_xml.value);
                        },
                        Field::Country => {
                            if country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            let country_xml: XmlStringValue = map.next_value()?;
                            country = Some(country_xml.value);
                        },
                        Field::WebAddress => {
                            if web_address.is_some() {
                                return Err(serde::de::Error::duplicate_field("webaddress"));
                            }
                            let web_address_xml: XmlStringValue = map.next_value()?;
                            web_address = Some(web_address_xml.value);
                        },
                        Field::XboxAccount => {
                            if xbox_account.is_some() {
                                return Err(serde::de::Error::duplicate_field("xboxaccount"));
                            }
                            let xbox_account_xml: XmlStringValue = map.next_value()?;
                            xbox_account = Some(xbox_account_xml.value);
                        },
                        Field::WiiAccount => {
                            if wii_account.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiiaccount"));
                            }
                            let wii_account_xml: XmlStringValue = map.next_value()?;
                            wii_account = Some(wii_account_xml.value);
                        },
                        Field::PsnAccount => {
                            if psn_account.is_some() {
                                return Err(serde::de::Error::duplicate_field("psnaccount"));
                            }
                            let psn_account_xml: XmlStringValue = map.next_value()?;
                            psn_account = Some(psn_account_xml.value);
                        },
                        Field::BattlenetAccount => {
                            if battlenet_account.is_some() {
                                return Err(serde::de::Error::duplicate_field("battlenetaccount"));
                            }
                            let battlenet_account_xml: XmlStringValue = map.next_value()?;
                            battlenet_account = Some(battlenet_account_xml.value);
                        },
                        Field::SteamAccount => {
                            if steam_account.is_some() {
                                return Err(serde::de::Error::duplicate_field("steamaccount"));
                            }
                            let steam_account_xml: XmlStringValue = map.next_value()?;
                            steam_account = Some(steam_account_xml.value);
                        },
                        Field::TradeRating => {
                            if trade_rating.is_some() {
                                return Err(serde::de::Error::duplicate_field("traderating"));
                            }
                            let trade_rating_xml: XmlIntValue = map.next_value()?;
                            trade_rating = Some(trade_rating_xml.value);
                        },
                        Field::Top => {
                            if top_list.is_some() {
                                return Err(serde::de::Error::duplicate_field("top"));
                            }
                            let top_list_xml: ItemList = map.next_value()?;
                            top_list = Some(top_list_xml.items);
                        },
                        Field::Hot => {
                            if hot_list.is_some() {
                                return Err(serde::de::Error::duplicate_field("hot"));
                            }
                            let hot_list_xml: ItemList = map.next_value()?;
                            hot_list = Some(hot_list_xml.items);
                        },
                        Field::Guilds => {
                            if guild_list.is_some() {
                                return Err(serde::de::Error::duplicate_field("guilds"));
                            }
                            guild_list = Some(map.next_value()?);
                        },
                        Field::Buddies => {
                            if buddy_list.is_some() {
                                return Err(serde::de::Error::duplicate_field("buddies"));
                            }
                            buddy_list = Some(map.next_value()?);
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let username = username.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let first_name =
                    first_name.ok_or_else(|| serde::de::Error::missing_field("firstname"))?;
                let last_name =
                    last_name.ok_or_else(|| serde::de::Error::missing_field("lastname"))?;
                let year_registered = year_registered
                    .ok_or_else(|| serde::de::Error::missing_field("yearregistered"))?;
                let last_login =
                    last_login.ok_or_else(|| serde::de::Error::missing_field("lastlogin"))?;
                let trade_rating =
                    trade_rating.ok_or_else(|| serde::de::Error::missing_field("traderating"))?;
                // Even if set to blank, empty string still included in response, so we set these to
                // none in this case
                if state_or_province.as_ref().is_some_and(String::is_empty) {
                    state_or_province = None;
                }
                if country.as_ref().is_some_and(String::is_empty) {
                    country = None;
                }
                if web_address.as_ref().is_some_and(String::is_empty) {
                    web_address = None;
                }
                if xbox_account.as_ref().is_some_and(String::is_empty) {
                    xbox_account = None;
                }
                if psn_account.as_ref().is_some_and(String::is_empty) {
                    psn_account = None;
                }
                if wii_account.as_ref().is_some_and(String::is_empty) {
                    wii_account = None;
                }
                if battlenet_account.as_ref().is_some_and(String::is_empty) {
                    battlenet_account = None;
                }
                if steam_account.as_ref().is_some_and(String::is_empty) {
                    steam_account = None;
                }
                Ok(Self::Value {
                    id,
                    username,
                    first_name,
                    last_name,
                    avatar_link,
                    year_registered,
                    last_login,
                    state_or_province,
                    country,
                    web_address,
                    xbox_account,
                    wii_account,
                    psn_account,
                    battlenet_account,
                    steam_account,
                    trade_rating,
                    top_list: top_list.unwrap_or(vec![]),
                    hot_list: hot_list.unwrap_or(vec![]),
                    buddies: buddy_list.unwrap_or(BuddyList::default()),
                    guilds: guild_list.unwrap_or(GuildList::default()),
                })
            }
        }
        deserializer.deserialize_any(UserVisitor)
    }
}
