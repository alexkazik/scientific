#!/bin/bash

#
# Perform a few simple checks ahead of a PR
#

# Usage: `./check.sh` or `./check.sh <toolchain>`
# If the toolchain is omitted `+nightly`,`+stable` and `+1.58.1` is used, `+stable` or `+beta` are the most common alternatives

TOOLCHAIN=${1:-+1.65.0}
echo Using toolchain $TOOLCHAIN

# use crates available at this rust version
cargo $TOOLCHAIN update

# builds (std+arc+debug+macro, std, nothing)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --features std --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features || exit 1

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds (std+arc+debug+macro, std, nothing)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --features std --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --tests || exit 1

# clippy (std+arc+debug+macro, std, nothing)
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features --features std --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# ensure that the library doesn't use re-exports
RUSTFLAGS="--cfg no_re_export" cargo build -p scientific

# update readme
( cd scientific && cargo rdme --force ) || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p scientific --all-features || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

TOOLCHAIN=${1:-+stable}
echo Using toolchain $TOOLCHAIN

# tests (std+arc+debug+macro, only debug [skip doctest since they require macro])
cargo $TOOLCHAIN test --locked --release --all-features -- --include-ignored || exit 1
cargo $TOOLCHAIN test --locked --release --no-default-features --features debug --lib --bins --tests -- --include-ignored || exit 1
