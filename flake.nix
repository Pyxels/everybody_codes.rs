{
  description = "Everybody Codes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        lib,
        system,
        pkgs,
        ...
      }: let
        git-hooks = import ./git-hooks.nix {
          inherit lib inputs pkgs system;
        };

        buildInputs = with pkgs; [
          cargo
          clippy
          rustfmt
        ];
      in {
        devShells = {
          # For `nix develop` / direnv
          default = pkgs.mkShell {
            packages = with pkgs;
              [
                bacon
                cargo-audit
                rust-analyzer
              ]
              ++ buildInputs;

            inherit (git-hooks.default) shellHook;
          };

          ci = pkgs.mkShell {
            inherit buildInputs;
          };
        };

        checks = {
          git-hooks = git-hooks.default;
        };
      };
    };
}
