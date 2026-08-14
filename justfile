#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt -- --config format_code_in_doc_comments=true

check:
    cargo check --all
    cargo check --all --all-features

clippy:
    cargo clippy --all
    cargo clippy --all --all-features

test:
    cargo test --all
    cargo test --all --all-features

precommit: fmt check clippy test
