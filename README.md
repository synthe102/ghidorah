# ghidorah
Wanna be multi-tenant Kong compatible dataplane.

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
