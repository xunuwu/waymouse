{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    nixpkgs,
    utils,
    crane,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      craneLib = crane.mkLib pkgs;

      waymouse-crate = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./.;
      };
    in {
      checks = {inherit waymouse-crate;};
      packages.default = waymouse-crate;
      apps.default = utils.lib.mkApp {drv = waymouse-crate;};
    });
}
