# GraphQL Sample / Benchmarks

This is an example graphQL server using actix, async-graphql, sqlx (postgres). TODO: add auth (via JWT), juniper and diesel (or another DB engine) to compare.

* api-server - actix http server that runs as docker image (e.g. on ecs or lambda)
* graphql - graphql server
* db - database access layer
* shared-types - types used by database and graphql

## Contributing

Copy `.env.sample` and rename `.env` to set up variables.

Install `sqlx-cli` 
`cargo install sqlx-cli --no-default-features --features postgres`

The sqlx macros require a database connection during build to validate the queries. You can avoid that by setting `SQLX_OFFLINE=true` in your .env file. If you make schema changes, run `cargo sqlx prepare` in the db folder to generate an updated file and check it into source control (and repeat in any folders that use sqlx query! macros).
## Writing Database Migrations

See the [Sqitch docs](https://sqitch.org/docs/manual), however, the general flow is as follows:

* **New change**: `sqitch add {change} -n 'A description of the changes' -r <change it depends on> -r <another dependency>`
* **Fix exiting migration (non-alter)**: `sqitch rework {change}`
  * if we've already deployed procedures and views, rework is an excellent option. It's silly to use for ALTER statements; just add new changes in those cases
    1. Update the deploy, revert, verify files created
    2. For the verify script, it must raise an EXCEPTION to fail (e.g. divide by zero, `ASSERT`, or `RAISE`)

 1. Proceed with development using `sqitch deploy --verify --target local` and `sqitch revert change^ --target local` for testing
 2. Once satisfied, tag the release! `sqitch tag <x.y.z>`  
 3. Then, DEPLOY!

## Deploying

TODO

### Deploying Migrations

First check the status using the following command  
`PGUSER=<user> PGPASSWORD='<password>' ./sqitch --target <[staging|prod]> status`  
Verify the changes that need to run are only the changes you expect.
If all is good then deploy the changes using the following command  
`PGUSER=<user> PGPASSWORD='<password>' ./sqitch --target <[staging|prod]> deploy`

*Note*  
If running against a local database then no need to use `PGUSER` and `PGPASSWORD` and `--target` as the default will point to the local database.

## Running locally

You need to have [docker](https://docs.docker.com/desktop/) and [lefthook](https://github.com/evilmartians/lefthook/blob/master/docs/full_guide.md) installed

### Running database

`lefthook run substrate`

To re-run the database migrations scripts and seed your local database.
`lefthook run down-substrate`
`lefthook run substrate`

### Running api server

You can run directly using something like the following, which uses the .env file:
`RUST_LOG=debug,sqlx=off,hyper=off cargo run --package api-server`
