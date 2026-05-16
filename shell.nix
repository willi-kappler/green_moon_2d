{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gnumake
    cmake
    pkg-config
    xmake
  ];

  buildInputs = with pkgs; [
    libGL
    libGL.dev
    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
  ];

  shellHook = ''
    echo "Development environment loaded for xmake + raylib!"
  '';
}

