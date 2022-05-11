{ system ? builtins.currentSystem
, pkgs ? import ../../nix { inherit system; }
}:

with pkgs;

let
  pypkgs = import ../../nix/pypkgs.nix { inherit pkgs; };
  gitbook-cli = import ../../nix/node/override.nix { inherit pkgs system; };
  doxybook2 = import ../../nix/doxybook2.nix { inherit pkgs; };
in
mkShell {
  buildInputs = [
    pkgs.doxygen
    pkgs.mdbook
    pkgs.sphinx
    pkgs.python39Packages.breathe
    python39Packages.sphinx_rtd_theme
    python39Packages.beautifulsoup4
    pypkgs.exhale
    xcbuild
    gitbook-cli
    doxybook2
  ];
}
