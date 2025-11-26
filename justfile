#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt -- --config format_code_in_doc_comments=true

check:
    cargo check
    cargo check --all-features

clippy:
    cargo clippy
    cargo clippy --all-features

test:
    cargo test
    cargo test --all-features

precommit: fmt check clippy test
