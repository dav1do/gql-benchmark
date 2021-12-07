-- Revert benchmarks:user from pg

BEGIN;

DROP FUNCTION IF EXISTS user_delete_fn;
DROP TRIGGER IF EXISTS user_soft_delete;
DROP TABLE IF EXISTS bench_user;

COMMIT;
