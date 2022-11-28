pub(crate) mod private {
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Ingestor {
        pub id:         Uuid,
        pub created_at: OffsetDateTime,
        pub name:       String,
        pub image_name: String,
        pub image_tag:  String,
    }

    impl From<model::Ingestor> for Ingestor {
        fn from(x: model::Ingestor) -> Self {
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
