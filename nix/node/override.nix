{ pkgs ? import <nixpkgs> { }
, system ? builtins.currentSystem
}:

let
  nodePackages = import ./default.nix {
    inherit pkgs system;
  };
  frameworks = pkgs.darwin.apple_sdk.frameworks;
in
nodePackages."gitbook-cli-2.3.2".override {
  buildInputs = [
    pkgs.nodePackages.node-gyp-build
  ]
  ++ pkgs.lib.optionals (system == "x86_64-darwin") [ frameworks.CoreServices ];
}
