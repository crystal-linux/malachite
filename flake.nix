{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec 
    {
      packages.malachite = naersk-lib.buildPackage {
        pname = "mlc";
        root = ./.;
      };
      
      packages.default = packages.malachite;

      apps.malachite = utils.lib.mkApp {
        drv = packages.malachite;
      };
      
      apps.default = apps.malachite;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          cargo-audit
          clippy
        ];
      };

      formatter = pkgs.alejandra;
    });
}
