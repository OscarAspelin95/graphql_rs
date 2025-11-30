-- Add down migration script here
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_updated_at;
DROP TABLE IF EXISTS public.users;
