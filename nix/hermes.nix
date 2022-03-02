{ pkgs ? import ./default.nix { } }:
let
  version = "v0.12.0";
  srcUrl = {
    x86_64-linux = {
      url =
        "https://github.com/informalsystems/ibc-rs/releases/download/${version}/hermes-${version}-x86_64-unknown-linux-gnu.tar.gz";
      sha256 = "sha256-pqunu7oy1MmNjgEyUXctjQRLNo0DMA+YDL8B48aayLs=";
    };
    x86_64-darwin = {
      url =
        "https://github.com/informalsystems/ibc-rs/releases/download/${version}/hermes-${version}-x86_64-apple-darwin.tar.gz";
      sha256 = "sha256-RMqWSN9BVzIzvzrrfpLKJMfhYMdbNfwNJwbnJx7I7mU=";
    };
  }.${pkgs.stdenv.system} or (throw
    "Unsupported system: ${pkgs.stdenv.system}");
in
pkgs.stdenv.mkDerivation {
  name = "hermes";
  inherit version;
  src = pkgs.fetchurl srcUrl;
  sourceRoot = ".";
  installPhase = ''
    echo "hermes"
    echo $out
    install -m755 -D hermes $out/bin/hermes
  '';

  meta = with pkgs.lib; { platforms = with platforms; linux ++ darwin; };

}
