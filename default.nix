{
  nixpkgs ? <nixpkgs>,
  nixpkgsMozilla ? fetchGit {
    url = https://github.com/mozilla/nixpkgs-mozilla;
    rev = "50bae918794d3c283aeb335b209efd71e75e3954";
  },
  system ? builtins.currentSystem,
  overlays ? [],
  crossSystem ? (import <nixpkgs/lib>).systems.examples.aarch64-multiplatform,
}:
let
  pkgs = import nixpkgs {
    inherit system crossSystem;
    overlays =
      let
        rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
        cargo2nixOverlay = import ../../cargo2nix/overlay;
      in
        overlays ++ [ cargo2nixOverlay rustOverlay ];
  };

  inherit (pkgs) lib buildPackages;

  rustPackageConfig = pkgs:
    let
      # A quick fix for missing frameworks errors on macOS.
      darwinFrameworks = lib.optionals pkgs.hostPlatform.isDarwin
        (with pkgs.darwin.apple_sdk.frameworks; [ Security CoreServices ]);
    in
      {
        rustcflags = {
          "registry+https://github.com/rust-lang/crates.io-index"."*" = [ "--cap-lints" "warn" ];
        };
        nativeBuildInputs = {
          unknown.gmp-mpfr-sys."*" =
            (with pkgs.buildPackages.buildPackages; [ gnum4 ])
            ++ darwinFrameworks;
        };
        buildInputs = {};
        environment = {
          # "registry+https://github.com/rust-lang/crates.io-index".openssl-sys."*" =
          #   let
          #     envize = s: builtins.replaceStrings ["-"] ["_"] (lib.toUpper s);
          #     envBuildPlatform = envize pkgs.buildPlatform.config;
          #     envHostPlatform = envize pkgs.hostPlatform.config;
          #     patchOpenssl = pkgs: (pkgs.openssl.override {
          #       # `perl` is only used at build time, but the derivation incorrectly uses host `perl` as an input.
          #       perl = pkgs.buildPackages.buildPackages.perl;
          #     }).overrideAttrs (drv: {
          #       installTargets = "install_sw";
          #       outputs = [ "dev" "out" "bin" ];
          #     });
          #     joinOpenssl = openssl: buildPackages.symlinkJoin {
          #       name = "openssl"; paths = with openssl; [ out dev ];
          #     };
          #   in
          #     # We don't use key literals here, as they might collide if `hostPlatform == buildPlatform`.
          #     builtins.listToAttrs [
          #       { name = "${envBuildPlatform}_OPENSSL_DIR"; value = joinOpenssl (patchOpenssl buildPackages); }
          #       { name = "${envHostPlatform}_OPENSSL_DIR"; value = joinOpenssl (patchOpenssl pkgs); }
          #     ];
        };
      };
  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    inherit rustPackageConfig;
    rustChannel = "1.37";
    packageFun = import ./Cargo.nix;
    localPatterns = [
      ''^(src)(/.*)?''
      ''[^/]*\.(rs|toml)$''
      ''^gmp-6\.2\.0-c(/.*)?''
      ''^mpc-1\.1\.0-c(/.*)?''
      ''^mpfr-4\.0\.2-p1-c(/.*)?''
    ];
  };
in
{
  package = rustPkgs.unknown.gmp-mpfr-sys."1.2.0" { };
}
