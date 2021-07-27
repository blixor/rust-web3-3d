{pkgs ? import <nixpkgs> {}}:

# https://github.com/kolloch/crate2nix/issues/111
with (import ./wasm/Cargo.nix {inherit pkgs;});
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    #rootCrate.build
    yarn
    clippy
  ];
  #shellHook = ''
  #yarn start
  #'';
}
