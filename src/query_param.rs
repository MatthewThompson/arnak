use chrono::NaiveDate;

use crate::{CollectionItemType, GameType, ItemType, WishlistPriority};

pub(crate) type QueryParam<'a> = (&'a str, String);

// Can't use the from and into traits because it needs to be defined for
// foreign types. But since it's not a public trait it doesn't really matter.
pub(crate) trait IntoQueryParam {
    fn into_query_param(self, key: &str) -> QueryParam<'_>;
}

impl IntoQueryParam for bool {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        match self {
            true => (key, "1".to_owned()),
            false => (key, "0".to_owned()),
        }
    }
}

// It would be good to just define this trait for all ToString, and then specialise the bool
// implementation. However, currently this seemingly doesn't work, the ever given being conflicting
// types with the bool implementation. I think this may be solvable with the default fn feature, but
// this is currently not stable.

impl IntoQueryParam for &str {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_owned())
    }
}

impl IntoQueryParam for u64 {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_string())
    }
}

impl IntoQueryParam for f32 {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_string())
    }
}

impl IntoQueryParam for NaiveDate {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.format("%y-%m-%d").to_string())
    }
}

impl IntoQueryParam for ItemType {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_string())
    }
}

impl IntoQueryParam for CollectionItemType {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_string())
    }
}

impl IntoQueryParam for GameType {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_string())
    }
}

impl IntoQueryParam for WishlistPriority {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        match self {
            WishlistPriority::DontBuyThis => (key, "5".to_owned()),
            WishlistPriority::ThinkingAboutIt => (key, "4".to_owned()),
            WishlistPriority::LikeToHave => (key, "3".to_owned()),
            WishlistPriority::LoveToHave => (key, "2".to_owned()),
            WishlistPriority::MustHave => (key, "1".to_owned()),
        }
    }
}

impl<Stringable: ToString> IntoQueryParam for Vec<Stringable> {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        let value_list = self
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");
        (key, value_list)
    }
}
