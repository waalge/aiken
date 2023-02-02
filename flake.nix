{
  # Based on the flakes:
  # - https://github.com/helix-editor/helix/blob/master/flake.nix
  # - https://github.com/yusdacra/nix-cargo-integration/blob/master/docs/example_flake.nix

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };
  outputs = inputs:
    inputs.nci.lib.makeOutputs {
      root = ./.;
      config = common: {
        outputs = {
          defaults = {
            package = "aiken";
            app = "aiken";
          };
        };
        shell = {
          name = "aiken-dev";
          packages = with common.pkgs; [
            rnix-lsp
          ];
        };
      };
    };
}
