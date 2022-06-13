{ sources ? import ./sources.nix, system ? builtins.currentSystem, ... }:

import sources.nixpkgs {
  overlays = [

    (_: pkgs: {
      pystarport = pkgs.poetry2nix.mkPoetryApplication {
        projectDir = sources.pystarport;
        src = sources.pystarport;
      };
    })

    (_: pkgs: { test-env = import ./testenv.nix { inherit pkgs; }; })

  ];
  config = { };
  inherit system;
}
