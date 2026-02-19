set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default: test

test:
    opz rpsn-dev -- cargo test

test-live-api:
    opz rpsn-dev -- cargo test live_api_ -- --ignored --test-threads=1 --nocapture

coverage-install:
    if ! cargo llvm-cov --version >/dev/null 2>&1; then cargo install cargo-llvm-cov; fi

coverage: coverage-install
    mkdir -p coverage
    opz rpsn-dev -- cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov.info

coverage-live-api: coverage-install
    mkdir -p coverage
    opz rpsn-dev -- cargo llvm-cov --workspace --all-features --lcov --output-path coverage/lcov-live.info -- --ignored live_api_ --test-threads=1 --nocapture

# Release new version (tag + push)
release-check:
    opz rpsn-dev -- cargo test --all --all-features
    opz rpsn-dev -- cargo build --release --all-features
    opz rpsn-dev -- cargo publish --dry-run

release: release-check
    version=$(rg -n "^version = " Cargo.toml | head -n1 | awk -F'"' '{print $2}'); \
    git tag "v${version}"; \
    git push origin "v${version}"
