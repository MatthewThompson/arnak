use serde::Deserialize;

/// A play is a recorded instance of someone playing a game. This struct includes one page of a list
/// of plays, along with the total number in the list.
#[derive(Clone, Debug, Deserialize)]
pub struct Plays {}
