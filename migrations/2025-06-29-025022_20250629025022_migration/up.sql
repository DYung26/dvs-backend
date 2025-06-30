-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	username VARCHAR NOT NULL,
	email VARCHAR NOT NULL unique,
	password VARCHAR NOT NULL
);

CREATE TABLE otps (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	user_id UUID NOT NULL unique,
	email VARCHAR NOT NULL unique,
	otp VARCHAR(6) NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
