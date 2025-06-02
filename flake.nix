{
  description = "Regalk.dev site (Rust + Leptos)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # nixpkgs 
    crane.url = "github:ipetkov/crane"; # cool tool for build rust projects on nix
    flake-utils.url = "github:numtide/flake-utils"; # flake-utils... yes
    rust-overlay.url = "github:oxalica/rust-overlay"; # rust overlay for rust 
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
          config.allowUnfree = true;
        }; # confg of the pkgs - allowUnFree - and add the rust-overlay 
        craneLib = (crane.mkLib pkgs).overrideToolchain
          (pkgs.rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          }); # tell cranelib to use the specific overlay option

        fullSrc = ./.; # src of the project

        depSrc = craneLib.cleanCargoSource fullSrc; # load the project and Cargo.toml for deps and build leptos

        cargoArtifacts = craneLib.buildDepsOnly {
          src = depSrc;
        }; # buildDepsOnly config see https://github.com/ipetkov/crane/issues/291
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
          ]; # pkgs for build the project

          buildPhase = ''
            # Set critical environment variables
            export LEPTOS_OUTPUT_NAME=regalk
            export LEPTOS_SITE_ROOT=$PWD/target/site
            cargo-leptos build --release
          ''; # Commands to build leptos app

          installPhase = ''
            mkdir -p $out/bin $out/site
            install -m755 target/release/regalk $out/bin/
            cp -r target/site/* $out/site
            cp -r blogs $out/blogs
          ''; # Copy the assets that app itself + blog folder to the $out dir

          meta = {
            description = "Rust + Leptos site for regalk.dev";
            mainProgram = "regalk.dev";
            maintainers = [ "regalk" ];
          }; # meta stuff
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
        }; # Stuff I use on the `nix develop` shell 
      }
    ) // {
      nixosModules.default = import ./module.nix self; # Module to wrap the app on a systemd service
    };
}