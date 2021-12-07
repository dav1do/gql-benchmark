FROM sqitch/sqitch:latest

USER root

ENV PGHOST=db_benchmarks \
    PGPORT=5432 \
    PGUSER=bench \ 
    PGPASSWORD=password \
    PGDATABASE=benchmarks

COPY migrations /repo/migrations

WORKDIR /repo/migrations




