{ pkgs ? import <nixpkgs> {} }:
pkgs.buildEnv {
  name = "for-transducers";
  paths = [
    # rust packages
    pkgs.rustc
    pkgs.cargo
    # gcc for building c and c++ code
    pkgs.gcc
    pkgs.gnumake
    # autoconf and m4 and all
    pkgs.autoconf
    pkgs.automake
    pkgs.libtool
    pkgs.pkg-config
    pkgs.m4
    # alt-ergo prover
    pkgs.alt-ergo
    # cvc5 prover
    pkgs.cvc5
    # z3 prover
    pkgs.z3
    # latex building environment
    # only light version 
    pkgs.texlive.combined.scheme-small
  ];
}
