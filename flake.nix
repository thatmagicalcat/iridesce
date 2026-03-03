{
  description = "Macroquad development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        buildDeps = with pkgs; [
          pkg-config
          alsa-lib
          libX11
          libXi
          libXcursor
          libXrandr
          libxkbcommon
          libGL
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ ];
          buildInputs = buildDeps;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildDeps;
        };
      }
    );
}
