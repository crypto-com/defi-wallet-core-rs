{ pkgs ? import ./default.nix { } }:
let
  pname = "cronosd";
  version = "0.6.5";
  srcUrl = {
    x86_64-linux = {
      url =
        "https://github.com/crypto-org-chain/cronos/releases/download/v${version}/cronos_${version}_Linux_x86_64.tar.gz";
      sha256 = "sha256-rwrDfEujtxc8sOTuKD0pPSB3PmcUe4Qc8jmtYVlrw00=";
    };
    x86_64-darwin = {
      url =
        "https://github.com/crypto-org-chain/cronos/releases/download/v${version}/cronos_${version}_Darwin_x86_64.tar.gz";
      sha256 = "sha256-l9AAW728xfQnRhHcqQN10oYlHZkib03RDHo9Kyh4PHc=";
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
