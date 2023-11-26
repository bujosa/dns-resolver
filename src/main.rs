use dns_resolver::service::dns_resolver::resolve_dns;
use serde::Deserialize;
use trust_dns_resolver::{config::ResolverConfig, TokioAsyncResolver};
use warp::Filter;

#[derive(Deserialize)]
struct UrlBody {
    url: String,
}

async fn handle_resolve(
    body: UrlBody,
    resolver: TokioAsyncResolver,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = resolve_dns(&body.url, &resolver).await;
    match result {
        Some(ip) => Ok(format!("The IP address of {} is: {}", body.url, ip)),
        None => Ok(format!("Could not resolve the IP address for {}", body.url)),
    }
}

#[tokio::main]
async fn main() {
    // Create a DNS resolver
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), Default::default());

    let resolver = warp::any().map(move || resolver.clone());

    let resolve_route = warp::post()
        .and(warp::path("resolve"))
        .and(warp::body::json())
        .and(resolver.clone())
        .and_then(handle_resolve);

    warp::serve(resolve_route).run(([127, 0, 0, 1], 3030)).await;
}
