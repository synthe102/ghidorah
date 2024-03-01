# ghidorah
Wanna be multi-tenant Kong compatible dataplane.

## Getting started

Generate and save the DP cert/key to `./auth/cert.pem` and `./auth/key.pem` which is the default location.

```bash
# Start ghidorah
cargo run -- --control-plane-url <kong-control-plane-url>

```

If using custom file names for cert/key:

```bash
# Start ghidorah
cargo run -- --control-plane-url <kong-control-plane-url> -c <cert-path> -k <key-path>

```

## Where we're going

- [x] Connect to Kong CP
- [x] Grab and parse configuration
- [x] Start pingora server
- [ ] Configure pingora server
- [ ] Graceful restart of pingora server on LB changes

## What about multi-tenancy ?

- [ ] Connect to multiple CP
- [ ] Merge configurations with fqdn based routing

This supposes that the CP verifies domain ownership to avoid config poisoning.
However if we ever get there it means that single-tenancy is working properly which is a huge goal in itself.

## Why ghidorah ?

Japanse Kaiju with multiple heads, like a gateway having multiple control planes.
