_: {
  projectRootFile = "flake.nix";

  programs = {
    nixfmt.enable = true;
    rustfmt.enable = true;
    yamlfmt.enable = true;
    prettier.enable = true;
  };
}
