# The importApply argument. Use this to reference things defined locally,
# as opposed to the flake where this is imported.
localFlake:

# Regular module arguments; self, inputs, etc all reference the final user flake,
# where this module was imported.
{
  lib,
  config,
  self,
  inputs,
  ...
}:
let
  resterrsMod = localFlake.moduleWithSystem (
    perSystem@{ config }: localFlake.importApply ./nixosModules perSystem
  );
in
{
  perSystem =
    { system, ... }:
    let
      resterrs' = localFlake.withSystem system (
        { config, pkgs, ... }: pkgs.callPackage ./pkgs/resterrs.nix { inherit inputs; }
      );
    in
    {
      packages.resterrs = resterrs';
      packages.default = resterrs';
    };

  flake.nixosModules.resterrs = resterrsMod;
  flake.nixosModules.default = resterrsMod;
}
