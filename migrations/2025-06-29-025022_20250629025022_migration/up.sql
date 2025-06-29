-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	username VARCHAR NOT NULL,
	email VARCHAR NOT NULL unique,
	password VARCHAR NOT NULL
);
