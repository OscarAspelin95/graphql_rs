-- Add up migration script here
CREATE TABLE public.users (
	id UUID PRIMARY KEY DEFAULT uuidv7(),
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Test add index.
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_updated_at ON users(updated_at);
