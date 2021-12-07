-- Deploy benchmarks:db to pg

-- XXX Add DDLs here.

BEGIN;

SET client_min_messages TO WARNING;

ALTER DATABASE benchmarks WITH CONNECTION_LIMIT=-1; 
-- ***************************************************;
--          Schema and Extensions
-- ***************************************************;
CREATE SCHEMA IF NOT EXISTS public;
CREATE EXTENSION IF NOT EXISTS pgcrypto;


CREATE OR REPLACE FUNCTION manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--call the manage function to automatically create a trigger on a new table
--select manage_updated_at('user'); 

COMMIT;
