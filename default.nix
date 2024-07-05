{ pkgs ? import <nixpkgs> {} }:
pkgs.buildEnv {
  name = "for-transducers";
  paths = [
    # rust packages
    pkgs.rustc
    pkgs.cargo
    # alt-ergo prover
    pkgs.alt-ergo
    # cvc5 prover
    pkgs.cvc5
    # z3 prover
    pkgs.z3
    # latex building environment
    # only light version 
    pkgs.texlive.combined.scheme-small
    # MONA
    (pkgs.callPackage ./MONA/mona.nix {})
    # tygiel
    # (pkgs.callPackage ./simplified_transducer/tygiel.nix {})
  ];
}
