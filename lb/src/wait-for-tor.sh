#!/bin/sh

# This script waits for Tor socks proxy to be available before starting loadbalanceeeer-rs.

TOR_SOCKS_PROXY="$1"
TOR_SOCKS_PROXY_HOST="$(echo "${TOR_SOCKS_PROXY}" | cut -d: -f1)"
TOR_SOCKS_PROXY_PORT="$(echo "${TOR_SOCKS_PROXY}" | cut -d: -f2)"
shift

until nc -z -w5 "${TOR_SOCKS_PROXY_HOST}" "${TOR_SOCKS_PROXY_PORT}"; do
  >&2 echo "Tor socks proxy (${TOR_SOCKS_PROXY}) is unavailable - sleeping"
  sleep 5
done

>&2 echo "Tor socks proxy (${TOR_SOCKS_PROXY}) is up - starting loadbalanceeeer-rs"
exec "$@"
