# Rust Database Proxy

A gRPC server, accepts queries and tenant IDs in requests, connects to the database associated with the tenant ID, executes the query, and returns the response set.

> Note regarding folder structure
> All of the work is done in main.rs at the moment, the structure is how it will, hopefully, eventually, be organized into, but it's easier to get it all working in one, simple file first.

## Connection

Connections to databases are established using ODBC via the `odbc_api` crate. Using `tokio`, the runtime is asynchronous, and connection pools are established to keep connections warm and latency low.

## Tenants

Currently, a `config.toml` file is used to define key value pairs for tenant IDs and their connection strings. This will be moved to a solution akin to HashiCorp's Vault, something cheaper or free though.

## Deployment

This server becomes much more useful in a Kubernets cluster with horizontal scaling, to add more nodes as the load increases and reduce the number of nodes as the load decreases. Perfect for managing lots of active connections at once, and saving resources during off peak hours.

## Testing

Run this server with `cargo run` for development testing, open a new terminal, and use `grpcurl` to send a request to the server:
```bash
grpcurl -plaintext -import-path ./proto -proto db_proxy.proto -d '{"tenant_id": "tenant1", "query": "SELECT * FROM users"}' '[::1]:50051' db_proxy.DbProxy/SendQuery
```

This is a very simple connection and request using the database `tenant1` which is in the repo. It's a `sqlite` database for testing.

## HTTP/1.1 Compatibility

Currently HTTP/1.1 compatibility is not turned on, but it's planned. This will have a fully documented gRPC and REST API for convienience. The proxy is primarily for managing connections from your backend, not a client, so gRPC is recommended, but on the off chance your backend stack does not support gRPC, REST will be available as a fallback.

### Note about gRPC in Progress OpenEdge

Progress does not officially support the gRPC protocol, but if an organization were to get in touch with Progress, a joint effort could be made to create a Progress gRPC library. It's a 10 year old technology, bleading edge yet already established and not 'new'. Progress needs support for this.

## Test Runner

A test runner is in development so this can be profiled and battle tested. Currently, only the initial test has been made, and it's behavior and performance under load is totally unknown.
