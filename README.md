# green_moon_2d

Green Moon 2D - a collection of useful data structures and functions for game development.

## BUILD:

To build the library just use the corresponding shell_*.nix environment.
For example:

```bash
nix-shell shell_gcc.nix
xmake -a -r
```

```bash
nix-shell shell_gcc.nix
meson setup builddir
meson compile -C builddir
```

This will download and install all dependencies automatically.

To run the tests:

```bash
nix-shell shell_gcc.nix
meson test -C builddir --v
```
