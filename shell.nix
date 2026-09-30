{ pkgs ? import <nixpkgs> {} }:

# pkgs.mkShell.override { stdenv = pkgs.llvmPackages_23.stdenv; } {
# pkgs.mkShell.override { stdenv = pkgs.llvmPackages.stdenv; } {
# pkgs.mkShell.override { stdenv = pkgs.gcc16Stdenv; } {
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cmake
    gnumake
    meson
    ninja
    pkg-config
    vcpkg
  ];

  buildInputs = with pkgs; [
    libGL
    libGL.dev
    libpng
    libX11
    libXcursor
    libXi
    libXinerama
    libXrandr
    nlohmann_json
    sdl3
    sdl3-image
    sdl3-mixer
    spdlog
    systemd
    zlib
  ];

  shellHook = ''
    echo "Development environment loaded for c++ and SDL3!"
  '';
}

