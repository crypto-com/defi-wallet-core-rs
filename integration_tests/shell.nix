{ system ? builtins.currentSystem, pkgs ? import ../nix { inherit system; } }:

with pkgs;

mkShell {
  buildInputs = [
    pkgs.pystarport
    pkgs.poetry
    pkgs.test-env
    pkgs.python39Packages.supervisor
    (import ../nix/testenv.nix { inherit pkgs; })
    (import ../nix/chainmain.nix { inherit pkgs; })
    (import ../nix/cronos.nix { inherit pkgs; })
    (import ../nix/hermes.nix { inherit pkgs; })
  ];
}
