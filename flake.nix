{
  description = "This flake is designed to be used ...";

  inputs = {
    flake-utils.url  = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        code = pkgs.callPackage ./. { inherit nixpkgs system rust-overlay; };
      in rec {
        packages = {
          all = pkgs.symlinkJoin {
            name = "all";
            paths = with code; [ ];
          };
          default = packages.all;
        };
      }
    );
}