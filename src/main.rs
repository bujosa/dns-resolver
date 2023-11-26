use std::net::IpAddr;
use trust_dns_resolver::{config::ResolverConfig, TokioAsyncResolver};

async fn resolve_dns(hostname: &str, resolver: &TokioAsyncResolver) -> Option<IpAddr> {
    match resolver.lookup_ip(hostname).await {
        Ok(response) => response.iter().next().map(|ip| ip),
        Err(err) => {
            eprintln!("Error resolving DNS: {:?}", err);
            None
        }
    }
}
#[tokio::main]
async fn main() {
    // Create a DNS resolver
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), Default::default());

    // Define the domain name you want to resolve
    let domain_name = "www.google.com";

    // Resolve the domain name and get the IP address
    let result = resolve_dns(domain_name, &resolver).await;

    // Handle the result
    match result {
        Some(ip) => println!("The IP address of {} is: {}", domain_name, ip),
        None => println!("Could not resolve the IP address for {}", domain_name),
    }
}
