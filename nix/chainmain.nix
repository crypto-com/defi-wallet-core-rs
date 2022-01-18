{ pkgs ? import ./default.nix { } }:
let
  pname = "chain-maind";
  version = "3.3.3";
  srcUrl = {
    x86_64-linux = {
      url =
        "https://github.com/crypto-org-chain/chain-main/releases/download/v${version}/chain-main_${version}_Linux_x86_64.tar.gz";
      sha256 = "sha256-5si15+ni5Biix2oktNhMCR0pZmQNgXknUU8dcAfr2a8=";
    };
    x86_64-darwin = {
      url =
        "https://github.com/crypto-org-chain/chain-main/releases/download/v${version}/chain-main_${version}_Darwin_x86_64.tar.gz";
      sha256 = "sha256-ri7TH+FSajRDIW1CC5R33XSj835wQgNnDXa2kPGi5C4=";
    };
  }.${pkgs.stdenv.system} or (throw
    "Unsupported system: ${pkgs.stdenv.system}");
in
pkgs.stdenv.mkDerivation {
  name = "${pname}";
  inherit version;
  src = pkgs.fetchurl srcUrl;
  sourceRoot = ".";
  installPhase = ''
    mkdir -p $out/bin
    cp bin/${pname} $out/bin/${pname}
    chmod +x $out/bin/${pname}
  '';

  meta = with pkgs.lib; { platforms = with platforms; linux ++ darwin; };

}
