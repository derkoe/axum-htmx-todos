-- Your SQL goes here
CREATE TABLE todos (
  id UUID PRIMARY KEY,
  title VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE,
  created_timestamp TIMESTAMP NOT NULL
)