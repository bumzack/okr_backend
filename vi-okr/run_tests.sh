#!/bin/bash

set -e

touch results.txt

docker compose -f impls/bep/compose.yaml up   --abort-on-container-exit
docker compose -f impls/bep-rust/compose.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_graalvm_java/compose.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_graalvm_java/compose.graal.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_graalvm_kotlin/compose.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_graalvm_kotlin/compose.graal.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_node_koa/compose.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_ref_impl_java21/compose.yaml up   --abort-on-container-exit
docker compose -f impls/gsc_rust_axum/compose.yaml up   --abort-on-container-exit


# does not work :-(
# docker compose -f impls/jha_ref_impl_dotnet/docker-compose.yml up  --abort-on-container-exit
