{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs = { self, nixpkgs, ... }:
  let
    pkgs = import nixpkgs {
      system = "x86_64-linux";
    };
    lib = pkgs.lib;
  in
  {
    defaultPackage.${pkgs.system} = pkgs.rustPlatform.buildRustPackage  {
      name = "wrestic";
      src = self;
      cargoHash = "sha256-XKhFey2QimyiVuhF0L61Quieaa+FtKoeu5tgKNvS0Ms=";

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

