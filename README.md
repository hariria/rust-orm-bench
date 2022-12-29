# Rust ORM Benchmark

The goal of this repo is to benchmark the performance and 
characteristics of different ORMs on Rust

### Context

Rust has several different ORMs (Object relational mappings) that 
make it easy for users to query or write data to their database.
Most of the time, these ORMs only support SQL DBs, Prisma being the
notable exception.

Some of the more popular ORMs include:

1. [Diesel](https://github.com/diesel-rs/diesel)
2. [Sea-ORM](https://github.com/SeaQL/sea-orm). 
3. [Prisma](https://github.com/Brendonovich/prisma-client-rust)

[SQLx](https://github.com/launchbadge/sqlx) is another popular 
option but is not considered an ORM, see 
[here](https://github.com/launchbadge/sqlx#sqlx-is-not-an-orm) 
for more details.

### Getting started

1. Edit the contents of the `src/constants.rs` file to include the absolute filepath of your database
2. Make sure all of the cargo dependencies are installed
3. Run `cargo prisma generate`
4. Run `cargo build` to build the project
5. Run `main.rs`. For those unfamiliar with rust you can also do `./target/debug/orm_bench` to run the executable. 


### Modifying benchmarks

Depending on what you're changing you may want the [Prisma CLI](https://www.prisma.io/docs/concepts/components/prisma-cli/installation)
to run db migrations or reset your db (I would install it globally to make your life easier but up to you).

### Benchmarks

At the moment I have 1 benchmark and it's only on a SQLITE db, 
looking at the amount of time it takes to perform 10k create / read /
delete ops in each of the ORMs that are being benchmarked. Each of 
the operations are done serially and without concurrent operations.

```bash
# Each of the Create / Find / Delete benchmarks below 
# were based on 10,000 records being added, found, or deleted 
# to a SQLITE DB serially, without any kind of concurrency

# Each record was related to a user, with a GUID as its PK
# and ID. For more info see the prisma/schema.prisma file

Running diesel benchmarks...
Create users elapsed 3.05s
Delete users elapsed 4.30s


Running prisma benchmarks...
Create users elapsed 8.13s
Find users elapsed 2.42s
Delete users elapsed 7.99s


Running prisma raw sql benchmarks...
Create users elapsed 4.14s
Find users elapsed 1.36s
Delete users elapsed 3.84s
```

I hope to add many more benchmarks soon. If you have any ideas
please add them to the discussions page on Github!
