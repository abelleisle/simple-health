CREATE TABLE goals (
    id uuid PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    calories_consumed INTEGER,
    calories_burned INTEGER,
    active_time_s INTEGER,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

select trigger_updated_at('goals');
