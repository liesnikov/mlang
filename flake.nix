{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [cargo2nix.overlays.default];
    };

    rustPkgs = pkgs.rustBuilder.makePackageSet {
      rustVersion = "1.61.0";
      packageFun = import ./Cargo.nix;
    };

    mlang = (rustPkgs.workspace.mlang {});

    workspaceshell = rustPkgs.workspaceShell {
      packages = [
        pkgs.rustc
        pkgs.cargo
        pkgs.rustfmt
        pkgs.rust-analyzer
        pkgs.clippy
      ];
    };

  in rec {
    packages = {
      mlang = mlang;
      default = packages.mlang;
    };

    apps = rec {
      mlang = { type = "app"; program = "${packages.default}/bin/mlangc"; };

      default = mlang;
    };

    devShells = {
      default = workspaceshell;
    };
  }
  );
}
