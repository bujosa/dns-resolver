# DNS Resolver

This project is a simple DNS resolver written in Rust. It uses the `trust-dns-resolver` crate to perform DNS lookups asynchronously.

## Features

- Asynchronous DNS resolution using the Tokio runtime.
- Error handling for failed DNS lookups.
- Prints the IP address of a given domain name.

## Example Usage

This DNS resolver runs as a web server and listens for POST requests on the `/resolve` endpoint. The POST request should contain a JSON body with a `url` field.

Run the server with the following command:

```bash
cargo run
```

Here's an example of how to use `curl` to send a POST request to the server:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"url":"www.google.com"}' http://localhost:3030/resolve
```

This will send a request to resolve the IP address of "www.google.com". The server will respond with a string containing the IP address or an error message.

## Dependencies
This project uses the following crates:
- `trust-dns-resolver` for DNS resolution.
- `tokio` for asynchronous programming.
- `warp` for web server functionality.
- `serde` for serialization and deserialization.

## Future Improvements
- Allow the user to input the domain name as a command-line argument.
- Implement more robust error handling.