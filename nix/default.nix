{ sources ? import ./sources.nix, system ? builtins.currentSystem, ... }:

import sources.nixpkgs {
  overlays = [

    (_: pkgs: {
      pystarport = pkgs.poetry2nix.mkPoetryApplication {
        projectDir = sources.pystarport;
        src = sources.pystarport;
      };
    })
    (_: pkgs:
      import ./scripts.nix {
        inherit pkgs;
        config = {
          chainmain-config = ../scripts/chainmain-devnet.yaml;
          cronos-config = ../scripts/cronos-devnet.yaml;
          hermes-config = ../scripts/hermes.toml;
          dotenv = builtins.path { name = "dotenv"; path = ../scripts/.env; };
        };
      })
    (_: pkgs: { test-env = import ./testenv.nix { inherit pkgs; }; })
  ];
  config = { };
  inherit system;
}
