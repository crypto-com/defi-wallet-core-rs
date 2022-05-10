{ system ? builtins.currentSystem
, pkgs ? import ../../nix { inherit system; }
}:

with pkgs;

mkShell {
  buildInputs = [
    pkgs.doxygen
    pkgs.mdbook
    pkgs.sphinx
    pkgs.python39Packages.breathe
    python39Packages.sphinx_rtd_theme
    python39Packages.beautifulsoup4
    pypkgs.exhale
    (import ../../nix/doxybook2.nix { inherit pkgs; })
  ];
}
