# Performance Test, Rust API Single Binary vs Separate Binaries Pipeline

## Run the api
1. Run as single binary:

```sh
    docker run -d -p8080:8080 erdivartanovich/rust-listdir-api
```
This command will spawn the container and run a single api binary called `list_dir_api`

2. Run as separate binaries:

```sh
    docker run -d -p8080:8080 -e BINARY=actix_api erdivartanovich/rust-listdir-api
```
This command will spawn the container and run `actix_api` as main entry point of the api.
The `actix_api` will execute and pass the request to another binary called `list_dir`, and then the execution result of `list_dir` binary will be piped back to `actix_api` and returned as response.
