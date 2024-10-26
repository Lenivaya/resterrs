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
    inputs@{
      nixpkgs,
      flake-parts,
      naersk,
      treefmt-nix,
      gitignore,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        treefmt-nix.flakeModule
        flake-parts.flakeModules.easyOverlay
      ];

      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          pkgs,
          self',
          lib,
          ...
        }:
        let
          naersk' = pkgs.callPackage naersk { };
          inherit (gitignore.lib) gitignoreSource;
          src = gitignoreSource ./.;
        in
        {
          overlayAttrs = {
            inherit (self'.packages) resterrs;
          };

          packages.resterrs = naersk'.buildPackage {
            inherit src;
            buildInputs = with pkgs; [
              pkg-config
              systemd
            ];
          };
          packages.default = self'.packages.resterrs;

          devShells =
            let
              bareMinimum = with pkgs; [
                rustc
                cargo

                systemd
                pkg-config
              ];
            in
            {
              default = pkgs.mkShell {
                name = "resterrs-dev";
                nativeBuildInputs =
                  with pkgs;
                  bareMinimum
                  ++ [
                    cargo-tarpaulin
                    cargo-edit

                    rustfmt
                    clippy

                    act
                  ];
                RUST_BACKTRACE = 1;
                RUST_LOG = "warn,test,info";
              };

              ci-tests = pkgs.mkShell {
                name = "resterrs-ci";
                nativeBuildInputs = bareMinimum ++ (with pkgs; [ cargo-tarpaulin ]);
                RUST_BACKTRACE = 1;
              };

              ci-format = pkgs.mkShell {
                name = "resterrs-ci-format";
                nativeBuildInputs =
                  bareMinimum
                  ++ (with pkgs; [
                    rustfmt
                    clippy
                  ]);
              };

              testing = pkgs.mkShell {
                name = "resterrs-test";
                nativeBuildInputs = [ self'.packages.default ];
              };
            };

          treefmt = {
            projectRootFile = "flake.nix";

            programs = {
              nixfmt.enable = true;
              rustfmt.enable = true;
              yamlfmt.enable = true;
              prettier.enable = true;
            };
          };
        };
    };
}
