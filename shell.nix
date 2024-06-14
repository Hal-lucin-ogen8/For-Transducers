{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    name = "for-transducers-shell";
    buildInputs = [ (import ./default.nix { inherit pkgs; }) ];
}
