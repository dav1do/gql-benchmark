-- Verify benchmarks:user on pg

BEGIN;

select id, created_at, updated_at, deleted_at from bench_user where false;

ROLLBACK;
