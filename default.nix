{ pkgs
, crane
, rust-bin
, src
}:

let
  craneLib = (crane.mkLib pkgs).overrideToolchain
    (rust-bin.nightly.latest.default.override {
      extensions = [ "rust-src" ];
      targets = [ "wasm32-unknown-unknown" ];
    });

in craneLib.buildPackage {
  inherit src;
  pname = "regalk";
  version = "0.1.0";

  buildInputs = with pkgs; [
    pkg-config
    openssl
  ];

  nativeBuildInputs = with pkgs; [
    dart-sass
    binaryen
    wasm-bindgen-cli
  ];

  buildPhase = ''
    export LEPTOS_OUTPUT_NAME=regalk
    cargo leptos build --release
  '';

  installPhase = ''
    mkdir -p $out/bin $out/site
    install -m755 target/release/regalk $out/bin/
    cp -r target/site/* $out/site
  '';
}