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

```shell
docker-compose up -d
scripts/integration-test.sh
docker-compose down
```