CREATE TABLE refresh_keys (
    user_id uuid PRIMARY KEY NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token text NOT NULL,
    expires_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

ALTER TABLE refresh_keys ADD CONSTRAINT refresh_keys_token_unique UNIQUE (token);
CREATE INDEX idx_refresh_keys_token ON refresh_keys (token);
select trigger_updated_at('refresh_keys');
