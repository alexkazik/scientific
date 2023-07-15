#!/bin/bash

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds (std+arc+debug, std, nothing)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --features std --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features || exit 1

# clippy (std+arc+debug, std, nothing)
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features --features std --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
( cd scientific && cargo rdme --force ) || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p scientific --all-features || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

# tests (std+arc+debug, nothing)
cargo $TOOLCHAIN test --release --all-features || exit 1
cargo $TOOLCHAIN test --release --all-features float_all -- --include-ignored || exit 1
cargo $TOOLCHAIN test --release --all-features integer_all -- --include-ignored || exit 1
cargo $TOOLCHAIN test --release --no-default-features --features debug || exit 1
cargo $TOOLCHAIN test --release --no-default-features --features debug float_all -- --include-ignored || exit 1
cargo $TOOLCHAIN test --release --no-default-features --features debug integer_all -- --include-ignored || exit 1
