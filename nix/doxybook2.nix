{ pkgs ? import ./default.nix { } }:
let
  pname = "doxybook2";
  version = "1.4.0";
  srcUrl = {
    x86_64-linux = {
      url =
        "https://github.com/matusnovak/doxybook2/releases/download/v${version}/doxybook2-linux-amd64-v${version}.zip";
      sha256 = "sha256-urk1b12qVQy/IdjZtVTqWci+A5cWosr26W3uUnE/zLA=";
    };
    x86_64-darwin = {
      url =
        "https://github.com/matusnovak/doxybook2/releases/download/v${version}/doxybook2-osx-amd64-v${version}.zip";
      sha256 = "sha256-nfFEpA8Yk0+qA0ESAMlkOqqEjHbPZLUxGsRuvQwdNHk=";
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
  nativeBuildInputs = [ pkgs.unzip ];
  meta = with pkgs.lib; { platforms = with platforms; linux ++ darwin; };
}
