{
  description = "an alternative to poweralertd";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};
      in {
        packages.default = naersk'.buildPackage {
          src = ./.;
        };
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              libnotify
              pkg-config
              cargo
              rustc
            ];
          };
      }
    );
}
