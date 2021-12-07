
CREATE DATABASE benchmarks
    WITH
    OWNER = david
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;

-- ***************************************************;
--          Schema and Extensions
-- ***************************************************;
CREATE EXTENSION IF NOT EXISTS pgcrypto;
