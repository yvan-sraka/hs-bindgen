{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ (import nixpkgs-mozilla) ];
        };
        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain;
          sha256 = "sha256-DzNEaW724O8/B8844tt5AVHmSjSQ3cmzlU4BP90oRlY=";
        }).rust;
      in {
        devShell = with pkgs;
          mkShell {
            buildInputs = [ libiconv toolchain ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}
