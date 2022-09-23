Procedural History Generator

Starts a http server at http://localhost:8080 and procedurally generates history
into a local MongoDB instance, which the user may navigate.

To generate entities, the program moves through each year, giving each type of
entity (land, settlement, person...) a turn to perform actions specified and
registered in the `events_*` files. These may create or modify other entities. 

Connection strings are hardcoded in `src/main.rs`

This code is not idiomatic Rust and the MongoDB queries are not optimized.
