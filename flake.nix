{
  description = "Regalk website (Rust + Leptos)";

  inputs = {
    nixpkgs       .url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay  .url = "github:oxalica/rust-overlay";
    flake-utils   .url = "github:numtide/flake-utils";
    crane         .url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };

      rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
        targets    = [ "wasm32-unknown-unknown" ];
      };

      craneLib = (import crane { inherit pkgs; }).overrideToolchain rustToolchain;


      regalk = craneLib.buildPackage {
        pname   = "regalk";
        version = "0.1.0";

        src = craneLib.cleanCargoSource ./.;

        nativeBuildInputs = [
          pkgs.pkg-config
          pkgs.dart-sass        # used by Leptos
          pkgs.binaryen         # wasm-opt
          pkgs.wasm-bindgen-cli # if you call it directly
          rustToolchain
        ];

        buildPhase = ''
          export LEPTOS_OUTPUT_NAME=regalk
          cargo leptos build --release --features=ssr
        '';

        installPhase = ''
          mkdir -p $out/bin $out/site
          install -m755 target/release/regalk $out/bin/
          cp -r target/site/*              $out/site/
          cp -r blogs rss.xml Cargo.toml   $out/   # extra runtime assets
        '';
      };
    in
    {
      packages.default = regalk;

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          pkg-config dart-sass binaryen pkgs.wasm-bindgen-cli
          rustToolchain
          pkgs.leptosfmt pkgs.cargo-generate pkgs.cargo-leptos pkgs.stylance-cli
        ];
      };

      nixosModules.default = { config, lib, pkgs, ... }: {
        options.services.regalk = with lib; {
          enable = mkEnableOption "Regalk website (Rust + Leptos)";

          address = mkOption {
            type    = types.str;
            default = "0.0.0.0:3000";
            description = "Bind address the server listens on";
          };
        };

        config = lib.mkIf config.services.regalk.enable {
          environment.systemPackages = [ regalk ];

          users.users.regalk = {
            isSystemUser = true;
            group = "regalk";
          };
          users.groups.regalk = { };

          systemd.services.regalk = {
            description = "Regalk Rust + Leptos website";
            after  = [ "network.target" ];
            wantedBy = [ "multi-user.target" ];

            serviceConfig = {
              ExecStart = "${regalk}/bin/regalk";
              Environment = [
                "LEPTOS_SITE_ROOT=${regalk}/site"
                "LEPTOS_SITE_ADDR=${config.services.regalk.address}"
              ];

              DynamicUser          = false;
              User                 = "regalk";
              Group                = "regalk";
              CapabilityBoundingSet= "";
              NoNewPrivileges      = true;
              ProtectSystem        = "strict";
              ProtectHome          = true;
              PrivateTmp           = true;
              PrivateDevices       = true;
              RestrictAddressFamilies = "AF_INET AF_INET6";
              SystemCallFilter     = "@system-service";
            };
          };
        };
      };
    });
}
