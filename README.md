# Sputnik

[![codecov](https://codecov.io/gh/sameerparekh/sputnik-golem/graph/badge.svg?token=AOQ4AGYL7R)](https://codecov.io/gh/sameerparekh/sputnik-golem)
[![CI](https://github.com/sameerparekh/sputnik-golem/actions/workflows/ci.yml/badge.svg)](https://github.com/sameerparekh/sputnik-golem/actions/workflows/ci.yml)

Sputnik is an exchange platform built with [Golem Cloud](https://golem.cloud/).

## Setup

1. [Install `protoc`](https://grpc.io/docs/protoc-installation/).
2. [Install `rustup`](https://rustup.rs/)
2. Install Rust & utilities

    ```shell
    rustup default stable
    cargo install golem-wasm-rpc-stubgen golem-cli cargo-component cargo-make
    ```

3. Install [dnsmasq](https://passingcuriosity.com/2013/dnsmasq-dev-osx/)

## Unit Testing

```shell
cargo test
```

## Integration Testing

1. Generate a private key:

```shell
cargo run --package chainmonitor -- gen-key --phrase 'seed phrase'
```

2. Copy .env.example to .env and update values appropriately.
3. Install `dnsmasq`.

   Configure `dnsmasq` so that all requests to `*.golem` resolve to localhost.

Note that integration testing is not currently working with Golem Cloud.

```shell
docker-compose up -d
scripts/integration-test.sh
docker-compose down
```