#!/bin/bash

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds (std+arc+debug+macro, std, nothing)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --features std --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features || exit 1

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

# tests (std+arc+debug+macro, only debug [skip doctest since they require macro])
cargo $TOOLCHAIN test --release --all-features -- --include-ignored || exit 1
cargo $TOOLCHAIN test --release --no-default-features --features debug --lib --bins --tests -- --include-ignored || exit 1
