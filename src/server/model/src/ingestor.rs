use diesel::Insertable;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::schema;

pub(crate) mod private {
    use diesel::Queryable;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Queryable, PartialEq, Eq, Clone, Debug)]
    pub struct Ingestor {
        pub id:         Uuid,
        pub created_at: OffsetDateTime,
        pub name:       String,
        pub image_name: String,
        pub image_tag:  String,
    }
}

#[derive(Insertable, PartialEq, Eq, Clone, Debug)]
#[diesel(table_name = schema::ingestor)]
pub struct Params<'a> {
    pub id:         Uuid,
    pub created_at: OffsetDateTime,
    pub name:       &'a str,
    pub image_name: &'a str,
    pub image_tag:  &'a str,
}
