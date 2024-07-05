{
    stdenv,
    rustc,
    cargo,
}:
stdenv.mkDerivation {
    pname   = "tygiel";
    version = "0.0.1";
    src     = ./.;
    buildInputs = [ rustc cargo ];
    buildPhase = ''
        cargo build --release
    '';
    installPhase = ''
        mkdir -p $out/bin
        cp target/release/tygiel $out/bin
    '';
}
