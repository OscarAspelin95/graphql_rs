# graphql_rs
Example of how to use graphql with Rust.

# Dependencies
axum for routing, etc.
serde for serialization/de-serialization.
tokio for all async stuff.
async-graphql.

# Components
### service.rs
We define a basic user with id, name, email, city and country.

### db.rs
We just use it for mocking data by implementing a struct that can return some made up users.

### query.rs
We define functionality to extract one user by id as well as extracting all users.

### main.rs
Main entrypoint. We make the main function async with tokio, create a routing with axum, add a tcp-listener with tokio and serve it with axum.
