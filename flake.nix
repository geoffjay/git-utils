{
  description = "git-utils flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = { url = "github:hercules-ci/gitignore.nix"; flake = false; };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, pre-commit-hooks, ... }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        gitignoreSrc = pkgs.callPackage inputs.gitignore { };
      in rec {
        packages.app = pkgs.callPackage ./default.nix { inherit gitignoreSrc; };

        legacyPackages = packages;

        defaultPackage = packages.app;

        devShells.default = pkgs.callPackage ./shell.nix {
          inherit pre-commit-hooks;
        };
      })
    );
}
