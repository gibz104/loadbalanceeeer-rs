# Loadbalanceeeer-rs

PoC local JSON-RPC load-balancer with anonymizer via Tor.  Built in Rust.

## Why

Distribute RPC requests across different RPC providers and use a proxy for privacy purposes. 

This is inspired by, and a fork of, [loadbalanceeeer](https://github.com/emilianobonassi/loadbalanceeeer) ...but written in Rust.  I hope to show others
that Rust can be quite simple, similar to Python, and much faster.  I'm new to Rust and found this fun to work on.

## How use it

- Install [Docker](https://docs.docker.com/get-docker/)

- Clone the repo

- Edit `docker-compose.yml` with your RPCs (currently some public examples from [Ethereum Nodes](https://ethereumnodes.com/))

- Run `docker-compose up -f docker-compose.yml`

- You get your new RPC at `http://localhost:9545`

## Architecture

Load Balancer with Anonymizer/Proxy

```
                                         => remote_rpc_1
                                        |
user => (localhost:9545) => (tor/proxy) -     ...
                                        |
                                         => remote_rpc_N
```

## Disclaimer

This is only to demonstrate how to load balance request across multiple providers and how tunnel them via a proxy for analysis, debugging (e.g. mitmproxy) and privacy purposes. 

Use at your own risks.