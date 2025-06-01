{
  description = "Regalk.dev site (Rust + Leptos)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
          config.allowUnfree = true;
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain
          (pkgs.rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          });

        fullSrc = ./.;

        depSrc = craneLib.cleanCargoSource fullSrc;

        cargoArtifacts = craneLib.buildDepsOnly {
          src = depSrc;
        };
      in
      {
        packages.default = craneLib.buildPackage {
          inherit cargoArtifacts;
          pname = "regalk";
          version = "0.1.0";
          src = fullSrc;

          nativeBuildInputs = with pkgs; [
            pkg-config
            dart-sass
            binaryen
            wasm-bindgen-cli
            cargo-leptos
          ];

          buildPhase = ''
            # Set critical environment variables
            export LEPTOS_OUTPUT_NAME=regalk
            export LEPTOS_SITE_ROOT=$PWD/target/site
            cargo-leptos build --release
          '';

          installPhase = ''
            mkdir -p $out/bin $out/site
            install -m755 target/release/regalk $out/bin/
            cp -r target/site/* $out/site
            cp -r blogs rss.xml Cargo.toml $out/
          '';

          meta = {
            description = "Rust + Leptos site for regalk.dev";
            mainProgram = "regalk.dev";
            maintainers = [ "regalk" ];
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];
          packages = with pkgs; [
            rust-analyzer
            tailwindcss-language-server
            typescript
            cargo-leptos
            cargo-generate
          ];
        };
      }
    ) // {
      nixosModules.default = import ./module.nix self;
    };
}