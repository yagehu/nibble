use serde::{Deserialize, Serialize};

pub(crate) mod private {
    use serde::{Deserialize, Serialize};
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
    pub struct Ingestor {
        pub id: Uuid,

        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,

        pub name:       String,
        pub image_name: String,
        pub image_tag:  String,
    }

    impl From<entity::Ingestor> for Ingestor {
        fn from(x: entity::Ingestor) -> Self {
            Self {
                id:         x.id,
                created_at: x.created_at,
                name:       x.name,
                image_name: x.image_name,
                image_tag:  x.image_tag,
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateRequestBody {
    pub name:       String,
    pub image_name: String,
    pub image_tag:  String,
}
