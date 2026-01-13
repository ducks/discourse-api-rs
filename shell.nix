{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "discourse-api";

  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    pkg-config
    openssl
    wasm-pack
    lld
  ];

  shellHook = ''
    echo "Discourse API - Rust Client Library"
    echo "===================================="
    echo ""
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
    echo "Commands:"
    echo "  cargo build              - Build the library"
    echo "  cargo test               - Run tests"
    echo "  cargo run --example fetch_latest - Run example"
    echo ""
  '';
}
