use crate::ItemType;

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

// TODO see if there is a way to define this trait for all ToString
// or ToString so we can define Display on enums and it'll "just work"
// Currently doesn't work due to conflicting types with the bool implementation.
// I think this may be solvable with the default fn feature, but this is currently
// not stable.

impl IntoQueryParam for &str {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_owned())
    }
}

impl IntoQueryParam for ItemType {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        (key, self.to_string())
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
