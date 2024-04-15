with import <nixpkgs> {};
mkShell {
  buildInputs = [ SDL2 SDL2_ttf SDL2_image SDL2_gfx SDL2_mixer ];
}
