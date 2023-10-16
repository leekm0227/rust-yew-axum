-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ITEMS (
	item_id SERIAL PRIMARY KEY,
    item_name VARCHAR(20) UNIQUE NOT NULL,
    item_type INTEGER NOT NULL,
    price INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    update_time TIMESTAMP,
    create_time TIMESTAMP DEFAULT NOW()
);

INSERT INTO ITEMS (item_name, item_type, price) VALUES ('츄르', 1, 1000);