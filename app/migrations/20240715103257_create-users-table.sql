CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(254) NOT NULL UNIQUE,
    password VARCHAR(128) NOT NULL
);
CREATE INDEX ix_users_email ON users(email);
