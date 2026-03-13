# PipedreamHQ/hudsucker Fork

**Upstream:** https://github.com/omjadas/hudsucker  
**Branch:** `per-connection-ca`  
**Purpose:** Per-connection CA certificate selection for multi-tenant MITM proxy

## What Changed

One breaking change to the `CertificateAuthority` trait — `gen_server_config` now receives the original CONNECT request:

```rust
// Before (upstream)
fn gen_server_config(&self, authority: &Authority) -> impl Future<Output = Arc<ServerConfig>> + Send;

// After (this branch)
fn gen_server_config(&self, authority: &Authority, connect_request: &Request<()>) -> impl Future<Output = Arc<ServerConfig>> + Send;
```

The built-in `RcgenAuthority` and `OpensslAuthority` accept and ignore the new parameter. Custom implementations can inspect headers (e.g., `Proxy-Authorization`) to select a different CA per connection.

## Files Changed (7 files, ~55 lines added)

- `src/certificate_authority/mod.rs` — trait signature + doc
- `src/certificate_authority/rcgen_authority.rs` — add `_connect_request` param
- `src/certificate_authority/openssl_authority.rs` — add `_connect_request` param
- `src/proxy/internal.rs` — build `Request<()>` from CONNECT request, pass to CA
- `benches/certificate_authorities.rs` — update call sites
- `benches/proxy.rs` — update call site
- `tests/common/mod.rs` — update call site

## Maintenance

The diff is minimal (~55 lines). To pull upstream changes:

```bash
git fetch upstream
git rebase upstream/main
```

All 24 existing tests pass on this branch.
