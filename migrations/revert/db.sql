-- Revert benchmarks:db from pg

BEGIN;

-- XXX Add DDLs here.
DROP EXTENSION IF EXISTS pgcrypto;
DROP SCHEMA IF EXISTS public cascade;
COMMIT;
