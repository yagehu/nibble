use serde::Deserialize;

pub(crate) mod private {
    use serde::Serialize;
    use time::OffsetDateTime;

    #[derive(Serialize, PartialEq, Eq, Clone, Debug)]
    pub struct Source {
        pub name: String,

        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateRequestBody {
    pub name: String,
}
