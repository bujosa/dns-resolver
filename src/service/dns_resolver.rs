use std::net::IpAddr;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn resolve_dns(hostname: &str, resolver: &TokioAsyncResolver) -> Option<IpAddr> {
    match resolver.lookup_ip(hostname).await {
        Ok(response) => response.iter().next().map(|ip| ip),
        Err(err) => {
            eprintln!("Error resolving DNS: {:?}", err);
            None
        }
    }
}
