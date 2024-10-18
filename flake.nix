{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?nixpkgs-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk' = pkgs.callPackage naersk { };
        binsider' = { release }: naersk'.buildPackage {
          name = "binsider";
          src = ./.;
          inherit release;
          meta = with pkgs.lib; {
            description = "Analyze ELF binaries like a boss";
            homepage = "https://binsider.dev/";
            license = [ licenses.mit licenses.asl20 ];
          };
        };
      in
      rec {
        packages = rec {
          binsider = binsider' { release = true; };
          binsider-debug = binsider' { release = false; };
          default = binsider;
        };
        checks.check = packages.binsider-debug;
      }
    );
}

