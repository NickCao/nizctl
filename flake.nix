{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = import nixpkgs { inherit system; overlays = [ self.overlay rust-overlay.overlay ]; }; in
        rec {
          packages = { inherit (pkgs) nizctl; };
          devShell = pkgs.mkShell { inputsFrom = [ pkgs.nizctl ]; };
        }
      ) //
    {
      overlay = final: prev:
        let
          toolchain = final.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            extensions = [ "rust-analyzer-preview" "rust-src" ];
          });
          platform = final.makeRustPlatform { cargo = toolchain; rustc = toolchain; };
        in
        {
          nizctl = platform.buildRustPackage {
            name = "nizctl";
            src = final.lib.cleanSource ./.;
            nativeBuildInputs = [ final.pkg-config ];
            buildInputs = [ final.hidapi final.libusb ];
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };
    };
}
