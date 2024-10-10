CREATE TABLE todo
(
    id          uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    title       TEXT UNIQUE NOT NULL,
    description TEXT,
    is_done     bool,
    created_at  timestamptz NOT NULL DEFAULT now(),
    updated_at  timestamptz
);

SELECT trigger_updated_at('"todo"');