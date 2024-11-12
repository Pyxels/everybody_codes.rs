{
  inputs,
  system,
  ...
}: let
  inherit (inputs) git-hooks;

  default-hooks = {
    src = ./.;
    hooks = {
      alejandra.enable = true;
      nil.enable = true;
      statix.enable = true;

      clippy = {
        enable = true;
        settings.denyWarnings = true;
      };
      rustfmt.enable = true;

      actionlint.enable = true;
      editorconfig-checker.enable = true;
    };
    settings.rust.cargoManifestPath = "2024/Cargo.toml";
  };
in {
  default = git-hooks.lib.${system}.run default-hooks;
}
