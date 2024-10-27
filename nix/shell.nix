{ pkgs, self' }:
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
}
