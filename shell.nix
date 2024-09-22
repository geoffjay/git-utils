{ pkgs, pre-commit-hooks }:
  let
    pre-commit-check = pre-commit-hooks.lib.${pkgs.system}.run {
      src = ./.;
      hooks = {
        # cargo-check = {
        #   enable = true;
        #   name = "cargo-check";
        #   description = "Check project";
        #   files = "\.rs$";
        #   entry = "${pkgs.cargo}/bin/cargo check --all";
        #   require_serial = true;
        #   pass_filenames = false;
        # };
        rustfmt.enable = true;
        # clippy.enable = true;
      };
    };
  in
  pkgs.mkShell {
    inherit (pre-commit-check) shellHook;

    CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

    packages = [
      pkgs.cargo
      pkgs.clippy
      pkgs.deno
      pkgs.ollama
      pkgs.overmind
      pkgs.rustc
      pkgs.rustfmt
    ];
  }
