CREATE TABLE ingestor
  ( id         uuid PRIMARY KEY
  , created_at timestamptz NOT NULL
  , name       TEXT NOT NULL
  , image_name TEXT NOT NULL
  , image_tag  TEXT NOT NULL
  );
