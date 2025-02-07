{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
      lib = pkgs.lib;
    in
    {
      defaultPackage.${pkgs.system} = pkgs.rustPlatform.buildRustPackage {
        name = "wrestic";
        src = self;
        useFetchCargoVendor = true;
        cargoHash = "sha256-QHCGUL2puzBShuudqiq29SS0JTKElaRptgBsAqq3q80=";

        meta = with lib; {
          homepage = "https://github.com/alvaro17f/wrestic";
          description = "wrestic";
          license = licenses.mit;
          maintainers = with maintainers; [ alvaro17f ];
          platforms = platforms.unix;
          #changelog = "https://github.com/alvaro17f/wrestic/blob/${version}/CHANGELOG.md";
          mainProgram = "wrestic";
        };
      };
    };
}
