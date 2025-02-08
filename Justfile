# Note: help messages should be 1 line long as required by just.

# Print a help message.
help:
    just --list

# Run `cargo hack --feature-powerset` on crates
powerset *args:
    # Group third-party implementation features to avoid a full combinatorial
    # explosion -- we assume that they build independent of each other.
    cargo hack --feature-powerset --workspace {{args}} --group-features newtype-uuid1,oxnet01,uuid1 --ignore-unknown-features

# Build docs for crates and direct dependencies
rustdoc *args:
    @cargo tree --depth 1 -e normal --prefix none --workspace --all-features \
        | gawk '{ gsub(" v", "@", $0); printf("%s\n", $1); }' \
        | xargs printf -- '-p %s\n' \
        | RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS='--cfg=doc_cfg' xargs cargo doc --no-deps --all-features {{args}}

# Generate README.md files using `cargo-sync-rdme`.
generate-readmes:
    # Please install via cargo install --locked --git https://github.com/sunshowers/cargo-sync-rdme for now.
    cargo sync-rdme --toolchain nightly --workspace --all-features
