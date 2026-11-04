{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          # TypeScript
          nodejs
          prettierd
          typescript-language-server
          eslint

          # DB Management + SQL formatter
          sqlitebrowser
          sqlite
          sleek

          # API Testing
          hurl

          # Rust
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer

          # Tauri
          pkg-config
          wrapGAppsHook4
          gtk3
          libsoup_3
          webkitgtk_4_1
        ];

        shellHook = ''
          export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH:$XDG_DATA_DIRS"
          export WEBKIT_DISABLE_DMABUF_RENDERER=1
        '';
      };
    };
}
