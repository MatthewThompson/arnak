pub(crate) type QueryParam<'a> = (&'a str, String);

// Can't use the from and into traits because it needs to be defined for
// foreign types. But since it's not a public trait it doesn't really matter.
pub(crate) trait IntoQueryParam {
    fn into_query_param(self, key: &str) -> QueryParam<'_>;
}
