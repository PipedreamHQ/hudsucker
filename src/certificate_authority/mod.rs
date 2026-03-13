#[cfg(feature = "openssl-ca")]
mod openssl_authority;
#[cfg(feature = "rcgen-ca")]
mod rcgen_authority;

use http::uri::Authority;
use hyper::Request;
use std::sync::Arc;
use tokio_rustls::rustls::ServerConfig;

#[cfg(feature = "openssl-ca")]
pub use openssl_authority::*;
#[cfg(feature = "rcgen-ca")]
pub use rcgen_authority::*;

const TTL_SECS: i64 = 365 * 24 * 60 * 60;
const CACHE_TTL: u64 = TTL_SECS as u64 / 2;
const NOT_BEFORE_OFFSET: i64 = 60;

/// Issues certificates for use when communicating with clients.
///
/// Clients should be configured to either trust the provided root certificate,
/// or to ignore certificate errors.
pub trait CertificateAuthority: Send + Sync + 'static {
    /// Generate ServerConfig for use with rustls.
    ///
    /// The `connect_request` parameter provides the original CONNECT request,
    /// allowing implementations to select different CA certificates based on
    /// request headers (e.g., `Proxy-Authorization`).
    fn gen_server_config(
        &self,
        authority: &Authority,
        connect_request: &Request<()>,
    ) -> impl Future<Output = Arc<ServerConfig>> + Send;
}
