-- Your SQL goes here
CREATE TABLE addresses (
  id SERIAL PRIMARY KEY,
  street_number VARCHAR NOT NULL,
  street_name VARCHAR NOT NULL,
  user_id integer NOT NULL REFERENCES users
)
