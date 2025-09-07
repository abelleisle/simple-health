CREATE TABLE users (
    id uuid PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    calorie_goal INTEGER NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

-- create unique index users_username_unique on users(lower(email));
select trigger_updated_at('users');

CREATE TABLE goals (
    id uuid PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    calories_consumed INTEGER,
    calories_burned INTEGER,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

select trigger_updated_at('goals');
