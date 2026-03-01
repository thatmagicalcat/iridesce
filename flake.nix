{
  description = "Rust dev environment for Raylib";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          # Tools needed at build time
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            clang
            libclang
          ];

          # Libraries that the code links against
          buildInputs = with pkgs; [
            libX11
            libXcursor
            libXrandr
            libXi
            libXinerama
            libGL
            wayland
            glfw
            libxkbcommon
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            libX11
            libXcursor
            libXrandr
            libXi
            libXinerama
            libGL
            wayland
            glfw
            libxkbcommon
            libclang
          ]);
        };
      }
    );
}
