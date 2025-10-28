

/// A user's information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct User {
    /// The ID of the user.
    pub id: u64,
    /// The username of the user.
    #[serde(rename = "name")]
    pub username: String,
    /// The first name of the user.
    pub first_name: String,
    /// The last name of the user.
    pub last_name: String,
    /// A link to the user's avatar.
    pub avatar_link: Option<String>,
    /// The year that the user registered on BoardGameGeek.
    pub year_registered: i64,
    /// The date and time that the user last logged in.
    pub last_login: DateTime<Utc>,
    /// 
    pub state_or_province: String,
    pub country: String,
    pub web_address: Option<String>,
    pub xbox_account: Option<String>,
    pub wii_account: Option<String>,
    pub psn_account: Option<String>,
    pub battlenet_account: Option<String>,
    pub steam_account: Option<String>,
}
