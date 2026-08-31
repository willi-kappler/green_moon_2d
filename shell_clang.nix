{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell.override { stdenv = pkgs.llvmPackages.stdenv; } {
  nativeBuildInputs = with pkgs; [
    gnumake
    cmake
    pkg-config
    vcpkg
    xmake
    ninja
  ];

  buildInputs = with pkgs; [
    libGL
    libGL.dev
    libX11
    libXrandr
    libXinerama
    libXcursor
    libXi
    sdl3
    sdl3-image
    sdl3-mixer
    spdlog
  ];

  shellHook = ''
    echo "Development environment loaded for c++ and SDL3!"
  '';
}

