use std::{sync::Arc, time::Duration};


use hickory_resolver::{Resolver, config::{ResolverConfig, ResolverOpts}, name_server::TokioConnectionProvider};

use crate::modules::Subdomain;

pub type dnsResolver = Arc<Resolver<TokioConnectionProvider>>;

pub async fn resolves(dnsresolver: &dnsResolver, domain: Subdomain) -> Option<Subdomain> {
    dnsresolver.lookup_ip(domain.domain.as_str())
        .await
        .ok()
        .map(|_| domain)
}

pub async fn new_resolver() -> dnsResolver{
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let resolver = Resolver::builder_with_config(ResolverConfig::default(), TokioConnectionProvider::default()).with_options(opts).build();

    return Arc::new(resolver);

}