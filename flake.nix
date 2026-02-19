{
  description = "Flake for Rust Projects";
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          # Graphic Library
          LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
            "${pkgs.xorg.libX11}/lib"
            "${pkgs.xorg.libXi}/lib"
            "${pkgs.libGL}/lib"
            "${pkgs.libxkbcommon}/lib"
          ];

          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy workshop-runner openssl];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
	  nativeBuildInputs = with pkgs; [
	      pkg-config
	  ];
	  dbus = pkgs.dbus;
        };
      }
    );
}
