{
  inputs,
  pkgs,
  ...
}:
let
  naersk' = pkgs.callPackage inputs.naersk { };
  inherit (inputs.gitignore.lib) gitignoreSource;
  src = gitignoreSource ../../../.;
in
naersk'.buildPackage {
  inherit src;
  buildInputs = with pkgs; [
    pkg-config
    systemd
  ];
}
