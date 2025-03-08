CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE,
    created_at TIMESTAMP
    WITH
        TIME ZONE DEFAULT CURRENT_TIMESTAMP,
        last_login TIMESTAMP
    WITH
        TIME ZONE
);

CREATE TABLE IF NOT EXISTS user_permissions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users (id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL,
    UNIQUE (user_id, token)
);

CREATE INDEX idx_user_permissions_user_id ON user_permissions (user_id);
