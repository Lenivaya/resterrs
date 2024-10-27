{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      {
        withSystem,
        moduleWithSystem,
        flake-parts-lib,
        ...
      }:
      let
        inherit (flake-parts-lib) importApply;
        resterrs-flake-module = importApply ./nix/resterrs {
          inherit withSystem moduleWithSystem importApply;
        };
      in
      {
        imports = [
          inputs.treefmt-nix.flakeModule
          inputs.flake-parts.flakeModules.easyOverlay
          resterrs-flake-module
        ];

        systems = [
          "x86_64-linux"
          "aarch64-linux"
        ];

        perSystem =
          {
            config,
            self',
            pkgs,
            lib,
            ...
          }:
          {
            overlayAttrs = {
              inherit (self'.packages) resterrs;
            };
            devShells = import ./nix/shell.nix { inherit pkgs self'; };
            treefmt = import ./nix/treefmt.nix { };
          };
      }
    );
}
