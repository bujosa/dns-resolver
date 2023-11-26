# DNS Resolver

This project is a simple DNS resolver written in Rust. It uses the `trust-dns-resolver` crate to perform DNS lookups asynchronously.

## Features

- Asynchronous DNS resolution using the Tokio runtime.
- Error handling for failed DNS lookups.
- Prints the IP address of a given domain name.

## Usage

To run the DNS resolver, simply execute the main function. By default, it will resolve the IP address for "www.google.com". 

```bash
cargo run
``` 

The output will either be the IP address of the domain or an error message indicating that the IP address could not be resolved.

## Customization
You can modify the domain_name variable in the main function to resolve the IP address for a different domain.

## Dependencies
This project uses the following crates:
- `trust-dns-resolver` for DNS resolution.
- `tokio` for asynchronous programming.

## Future Improvements
- Allow the user to input the domain name as a command-line argument.
- Implement more robust error handling.