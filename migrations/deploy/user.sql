-- Deploy benchmarks:user to pg
BEGIN;

SET client_min_messages TO WARNING;

CREATE TABLE IF NOT EXISTS bench_user (
    id uuid NOT NULL PRIMARY KEY,
    created_at timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at timestamptz NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at timestamptz
);


CREATE OR REPLACE FUNCTION public.user_delete_fn ()
    RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE
        bench_user
    SET
        is_deleted = current_timestamp
    WHERE
        id = OLD.id;
    RETURN NULL;
END;
$$;

DROP TRIGGER IF EXISTS user_soft_delete ON public.bench_user;
CREATE TRIGGER user_soft_delete
    BEFORE DELETE ON public.bench_user
    FOR EACH ROW
    EXECUTE PROCEDURE public.user_delete_fn ();


DO $$
BEGIN
    PERFORM
        manage_updated_at ('bench_user');
END
$$;

COMMIT;

