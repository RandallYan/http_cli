# CLI for HTTP Requests

This is a simple command-line interface (CLI) for making HTTP requests using the http module in Rust. It allows you to easily send GET and POST requests to a specified URL.

## Usage

To use this CLI, you will need to have Rust installed on your machine. Once you have Rust installed, you can run the following commands to use this CLI:

```shell
$ git clone https://github.com/RandallYan/http_cli
$ cd <repo-name>
$ cargo build
$ cargo run -- [get|post] <url> [body]
```

The cargo build command will build the executable, and cargo run will run the executable with the specified command-line arguments.

## Commands

This CLI supports the following commands:

### get <url>

Sends a GET request to the specified URL.

### post <url> <body>

Sends a POST request to the specified URL with the specified request body.

## Testing

To run the tests for this CLI, run the following command:

```rust
$ cargo test
```