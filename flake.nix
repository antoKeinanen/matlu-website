{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          bacon
          wrangler
          openssl
          lld
          cargo-watch
          simple-http-server
          rust-analyzer

          (pkgs.writeShellApplication {
            name = "start-dev-cdn";
            text = ''
              simple-http-server --cors --coop --coep -i ./assets              
            '';
          })
          (pkgs.writeShellApplication {
            name = "start-dev-server";
            text = ''
              cargo watch -w src -w templates -s "wrangler dev"
            '';
          })

        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
        '';
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
}
