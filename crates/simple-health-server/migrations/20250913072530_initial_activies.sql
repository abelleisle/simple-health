CREATE TABLE activities (
    id uuid PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    description VARCHAR(255),
    calories INTEGER NOT NULL,
    duration INTERVAL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

select trigger_updated_at('activities');
