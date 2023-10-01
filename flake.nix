{
  description = "getweb devShell and package";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            # rust-bin.stable.latest.default
            (rust-bin.stable.latest.default.override {
              targets = [
                "armv7-linux-androideabi"
                "aarch64-linux-android"
                "x86_64-pc-windows-gnu"
              ];
            })
            clippy
            rust-analyzer
            wine64
            pkgsCross.mingwW64.stdenv.cc
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
        packages.getweb = 
            rustPlatform.buildRustPackage {
                name = "getweb";
                src = ./.;
                cargoLock = {
                    lockFile = ./Cargo.lock;
                };
                OPENSSL_DEV=openssl.dev;
                OPENSSL_LIB_DIR = "${lib.getLib openssl}/lib";
                OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
                PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig"
                RUST_BACKTRACE = 1;
                buildInputs = [
                    (rust-bin.stable.latest.default.override {
                    targets = [
                        "armv7-linux-androideabi"
                        "aarch64-linux-android"
                        "x86_64-pc-windows-gnu"
                    ];
                    })
                    perl
                    pkg-config
                    openssl
                ];
                nativeBuildInputs = [
                    pkg-config
                    openssl
                ];
            };
             }
    );
}
