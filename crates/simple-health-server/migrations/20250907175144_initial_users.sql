CREATE TABLE users (
    id uuid PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

-- create unique index users_username_unique on users(lower(email));
select trigger_updated_at('users');
