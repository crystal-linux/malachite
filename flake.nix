{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec
      {
        packages.malachite = naersk-lib.buildPackage {
          pname = "Malachite";
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
            cargo-audit
            rustfmt
            clippy
            rust-analyzer

            # For `alpm` libs
            pkg-config
            pacman
            openssl
          ];
          # For rust-analyzer
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        formatter = pkgs.alejandra;
      });
}