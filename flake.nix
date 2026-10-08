{
  description = "drop";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "drop";
          version = "1.0.0";
          src = self;

          configurePhase = ''
            mkdir build
            cd build
            cmake .. -DCMAKE_SKIP_RPATH=1
          '';

          buildInputs = with pkgs; [
            stdenv.cc.cc
            cmake
          ];

          installPhase = ''
            mkdir -p "$out/bin"
            mkdir -p "$out/lib"

            cp drop "$out/bin"
            cp *.so "$out/lib"
            cp *.a "$out/lib"
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            stdenv.cc.cc
            cmake
          ];
        };
      }
    );
}
