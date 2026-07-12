let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # TypeScript
    nodejs
    prettierd
    typescript-language-server
    eslint

    # DB Management + SQL formatter
    sqlitebrowser
    sqlite
    sleek

    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer

    # Tauri
    pkg-config
    wrapGAppsHook4
  ];

  buildInputs = with pkgs; [
    librsvg
    webkitgtk_4_1
  ];

  shellHook = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH"
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
  '';
}
