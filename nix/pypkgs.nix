{ pkgs }:
pkgs.python39Packages.override {
  overrides = self: super: {
    exhale = super.buildPythonPackage rec {
      pname = "exhale";
      # 0.3+ needs breath >= 4.32.0
      version = "0.2.4";
      src = super.fetchPypi {
        inherit pname version;
        sha256 = "sha256-vD/1rXLLl8NGiUTB8OnQfmHDhrq6lzF625s4eEtrJpg=";
      };
      buildInputs = with super;
        [ breathe lxml beautifulsoup4 ];
    };
  };
}
