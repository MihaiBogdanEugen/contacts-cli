CREATE TABLE contacts (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  phone_no BIGINT NOT NULL,
  email TEXT NOT NULL
);