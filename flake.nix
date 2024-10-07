{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = rec {
          binsider = pkgs.rustPlatform.buildRustPackage {
            name = "binsider";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            checkType = "debug";
            checkFlags = [
              "--skip=test_extract_strings"
              "--skip=test_init"
            ];
            meta = with pkgs.lib; {
              description = "Analyze ELF binaries like a boss";
              homepage = "https://binsider.dev/";
              license = licenses.mit;
            };
          };
          default = binsider;
        };
      }
    );

}
