{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        binsider' = { buildType }: pkgs.rustPlatform.buildRustPackage {
          name = "binsider";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          checkType = "debug";
          inherit buildType;
          checkFlags = [
            "--skip=test_extract_strings"
            "--skip=test_init"
          ];
          meta = with pkgs.lib; {
            description = "Analyze ELF binaries like a boss";
            homepage = "https://binsider.dev/";
            license = [ licenses.mit licenses.asl20 ];
          };
        };
      in
      rec {
        packages = rec {
          binsider = binsider' { buildType = "release"; };
          binsider-debug = binsider' { buildType = "debug"; };
          default = binsider;
        };
        checks.check = packages.binsider-debug;

      }
    );
}
