{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [
      (python312.withPackages(ps: with ps; [
        flake8
        ipython
        mypy
        #python-lsp-ruff
        raylib
        ]))
      ruff
    ];
}

