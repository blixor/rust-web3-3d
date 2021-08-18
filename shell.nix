{pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    cargo-watch
    yarn
    clippy
  ];
}
