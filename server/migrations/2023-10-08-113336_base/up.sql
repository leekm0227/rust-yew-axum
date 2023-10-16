-- Your SQL goes here
CREATE TABLE IF NOT EXISTS USERS (
	user_id SERIAL PRIMARY KEY,
    account VARCHAR(15) UNIQUE NOT NULL,
	password VARCHAR(20) NOT NULL,
    nickname VARCHAR(15),
    coin INTEGER NOT NULL DEFAULT 0,
    refresh_token VARCHAR(40),
    last_login_time TIMESTAMP,
    update_time TIMESTAMP,
    create_time TIMESTAMP DEFAULT NOW()
);