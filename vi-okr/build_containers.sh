#!/bin/bash

set -e

docker compose -f impls/bep/compose.yaml build
docker compose -f impls/bep-rust/compose.yaml build
docker compose -f impls/gsc_graalvm_java/compose.yaml build
docker compose -f impls/gsc_graalvm_java/compose.graal.yaml build
docker compose -f impls/gsc_graalvm_kotlin/compose.yaml build
docker compose -f impls/gsc_graalvm_kotlin/compose.graal.yaml build
docker compose -f impls/gsc_node_koa/compose.yaml build
docker compose -f impls/gsc_ref_impl_java21/compose.yaml build
docker compose -f impls/gsc_rust_axum/compose.yaml build

# does not pass health check :-()
# docker compose -f impls/jha_ref_impl_dotnet/docker-compose.yml build

docker compose -f impls/jha_ref_impl_dotnet/compose.yml build
