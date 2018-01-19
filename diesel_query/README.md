# Diesel & Postgres

I want to see what is involved in working with postgres using diesel.

## Goals

- [x] [Create a migration](migrations/2018-01-04-040323_create_users)
- [x] [Insert data into a simple table](src/bin/create_user.rs)
- [x] [Query a simple table](src/bin/list_users.rs)
- [x] [Update data in a simple table](src/bin/update_user.rs)
- [x] [Join two tables](src/bin/list_users_with_addresses_joined.rs)
- [x] [Associate two tables](src/bin/list_users_with_addresses.rs)
- [x] [Load data in batches](src/bin/list_users_batched.rs)
- [ ] Load data in batches in a generic way.
- [ ] Use a transaction.
- [ ] Use a postgres extension type (e.g. hstore/postgis).
- [ ] Use a custom filter (e.g. users.filter(adults_only()).

## Notes

A lot I need to learn here to be as productive as I am with Active Record.
