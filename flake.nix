{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = import nixpkgs { inherit system; overlays = [ self.overlay ]; }; in
        {
          packages = { inherit (pkgs) nizctl niz-qmk-configurator; };
          devShell = pkgs.mkShell { inputsFrom = [ pkgs.nizctl ]; };
          defaultPackage = pkgs.nizctl;
        }
      ) //
    {
      overlay = final: prev: {
        nizctl = final.rustPlatform.buildRustPackage {
          name = "nizctl";
          src = final.lib.cleanSource ./.;
          nativeBuildInputs = [ final.pkg-config ];
          buildInputs = [ final.hidapi final.libusb ];
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        niz-qmk-configurator =
          final.callPackage ./niz-qmk-configurator.nix { };
      };
    };
}
