{
  release ? true,
  rootFeatures ? [
    "gmp-mpfr-sys/default" 
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  mkRustCrate,
  rustLib,
  lib,
}:
let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName (attrs: mkRustCrate (f attrs));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  "registry+https://github.com/rust-lang/crates.io-index".arrayref."0.3.5" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "arrayref";
    version = "0.3.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "arrayref";
      version = "0.3.5";
      sha256 = "0d382e583f07208808f6b1249e60848879ba3543f57c32277bf52d69c2f0f0ee";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".arrayvec."0.5.1" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "arrayvec";
    version = "0.5.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "arrayvec";
      version = "0.5.1";
      sha256 = "cff77d8686867eceff3105329d4698d96c2391c176d5d03adc90c7389162b5b8";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".autocfg."0.1.7" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "autocfg";
    version = "0.1.7";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "autocfg";
      version = "0.1.7";
      sha256 = "1d49d90015b3c36167a20fe2810c5cd875ad504b39cff3d4eae7977e6b7c1cb2";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".base64."0.11.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "base64";
    version = "0.11.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "base64";
      version = "0.11.0";
      sha256 = "b41b7ea54a0c9d92199de89e20e58d49f02f8e699814ef3fdf266f6f748d15c7";
    };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".blake2b_simd."0.5.10" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "blake2b_simd";
    version = "0.5.10";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "blake2b_simd";
      version = "0.5.10";
      sha256 = "d8fb2d74254a3a0b5cac33ac9f8ed0e44aa50378d9dbb2e5d83bd21ed1dc2c8a";
    };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
      arrayref = rustPackages."registry+https://github.com/rust-lang/crates.io-index".arrayref."0.3.5" { inherit profileName; };
      arrayvec = rustPackages."registry+https://github.com/rust-lang/crates.io-index".arrayvec."0.5.1" { inherit profileName; };
      constant_time_eq = rustPackages."registry+https://github.com/rust-lang/crates.io-index".constant_time_eq."0.1.5" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cfg-if."0.1.10" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "cfg-if";
    version = "0.1.10";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "cfg-if";
      version = "0.1.10";
      sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".constant_time_eq."0.1.5" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "constant_time_eq";
    version = "0.1.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "constant_time_eq";
      version = "0.1.5";
      sha256 = "245097e9a4535ee1e3e3931fcfcd55a796a44c643e8596ff6566d68f09b87bbc";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".crossbeam-utils."0.7.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "crossbeam-utils";
    version = "0.7.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "crossbeam-utils";
      version = "0.7.0";
      sha256 = "ce446db02cdc3165b94ae73111e570793400d0794e46125cc4056c81cbb039f4";
    };
    features = builtins.concatLists [
      [ "default" ]
      [ "lazy_static" ]
      [ "std" ]
    ];
    dependencies = {
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."0.1.10" { inherit profileName; };
      lazy_static = rustPackages."registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.4.0" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."0.1.7" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".dirs."2.0.2" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "dirs";
    version = "2.0.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "dirs";
      version = "2.0.2";
      sha256 = "13aea89a5c93364a98e9b37b2fa237effbb694d5cfe01c5b70941f7eb087d5e3";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."0.1.10" { inherit profileName; };
      dirs_sys = rustPackages."registry+https://github.com/rust-lang/crates.io-index".dirs-sys."0.3.4" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".dirs-sys."0.3.4" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "dirs-sys";
    version = "0.3.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "dirs-sys";
      version = "0.3.4";
      sha256 = "afa0b23de8fd801745c471deffa6e12d248f962c9fd4b4c33787b055599bde7b";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."0.1.10" { inherit profileName; };
      ${ if hostPlatform.isUnix then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.66" { inherit profileName; };
      ${ if hostPlatform.parsed.kernel.name == "redox" then "redox_users" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".redox_users."0.3.4" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.8" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".getrandom."0.1.14" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "getrandom";
    version = "0.1.14";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "getrandom";
      version = "0.1.14";
      sha256 = "7abc8dd8451921606d809ba32e95b6111925cd2906060d2dcc29c070220503eb";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."0.1.10" { inherit profileName; };
      ${ if hostPlatform.isUnix then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.66" { inherit profileName; };
      ${ if hostPlatform.parsed.kernel.name == "wasi" then "wasi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".wasi."0.9.0+wasi-snapshot-preview1" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "unknown".gmp-mpfr-sys."1.2.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "gmp-mpfr-sys";
    version = "1.2.0";
    registry = "unknown";
    src = fetchCrateLocal ./.;
    features = builtins.concatLists [
      (lib.optional (rootFeatures' ? "gmp-mpfr-sys/cnodelete") "cnodelete")
      (lib.optional (rootFeatures' ? "gmp-mpfr-sys/default") "default")
      (lib.optional (rootFeatures' ? "gmp-mpfr-sys/fail-on-warnings") "fail-on-warnings")
      (lib.optional (rootFeatures' ? "gmp-mpfr-sys/default" || rootFeatures' ? "gmp-mpfr-sys/mpc") "mpc")
      (lib.optional (rootFeatures' ? "gmp-mpfr-sys/default" || rootFeatures' ? "gmp-mpfr-sys/mpc" || rootFeatures' ? "gmp-mpfr-sys/mpfr") "mpfr")
      (lib.optional (rootFeatures' ? "gmp-mpfr-sys/use-system-libs") "use-system-libs")
    ];
    dependencies = {
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.66" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
      dirs = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".dirs."2.0.2" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".lazy_static."1.4.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "lazy_static";
    version = "1.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "lazy_static";
      version = "1.4.0";
      sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.66" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "libc";
    version = "0.2.66";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "libc";
      version = "0.2.66";
      sha256 = "d515b1f41455adea1313a4a2ac8a8a477634fbae63cc6100e3aebb207ce61558";
    };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".redox_syscall."0.1.56" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "redox_syscall";
    version = "0.1.56";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "redox_syscall";
      version = "0.1.56";
      sha256 = "2439c63f3f6139d1b57529d16bc3b8bb855230c8efcc5d3a896c8bea7c3b1e84";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".redox_users."0.3.4" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "redox_users";
    version = "0.3.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "redox_users";
      version = "0.3.4";
      sha256 = "09b23093265f8d200fa7b4c2c76297f47e681c655f6f1285a8780d6a022f7431";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
      getrandom = rustPackages."registry+https://github.com/rust-lang/crates.io-index".getrandom."0.1.14" { inherit profileName; };
      syscall = rustPackages."registry+https://github.com/rust-lang/crates.io-index".redox_syscall."0.1.56" { inherit profileName; };
      argon2 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".rust-argon2."0.7.0" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".rust-argon2."0.7.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "rust-argon2";
    version = "0.7.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "rust-argon2";
      version = "0.7.0";
      sha256 = "2bc8af4bda8e1ff4932523b94d3dd20ee30a87232323eda55903ffd71d2fb017";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
      base64 = rustPackages."registry+https://github.com/rust-lang/crates.io-index".base64."0.11.0" { inherit profileName; };
      blake2b_simd = rustPackages."registry+https://github.com/rust-lang/crates.io-index".blake2b_simd."0.5.10" { inherit profileName; };
      constant_time_eq = rustPackages."registry+https://github.com/rust-lang/crates.io-index".constant_time_eq."0.1.5" { inherit profileName; };
      crossbeam_utils = rustPackages."registry+https://github.com/rust-lang/crates.io-index".crossbeam-utils."0.7.0" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".wasi."0.9.0+wasi-snapshot-preview1" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "wasi";
    version = "0.9.0+wasi-snapshot-preview1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "wasi";
      version = "0.9.0+wasi-snapshot-preview1";
      sha256 = "cccddf32554fecc6acb585f82a32a72e28b48f8c4c1883ddfeeeaa96f7d8e519";
    };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.8" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "winapi";
    version = "0.3.8";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "winapi";
      version = "0.3.8";
      sha256 = "8093091eeb260906a183e6ae1abdba2ef5ef2257a21801128899c3fc699229c6";
    };
    features = builtins.concatLists [
      [ "knownfolders" ]
      [ "objbase" ]
      [ "shlobj" ]
      [ "winbase" ]
      [ "winerror" ]
    ];
    dependencies = {
      ${ if hostPlatform.config == "i686-pc-windows-gnu" then "winapi_i686_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" { inherit profileName; };
      ${ if hostPlatform.config == "x86_64-pc-windows-gnu" then "winapi_x86_64_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" { inherit profileName; };
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "winapi-i686-pc-windows-gnu";
      version = "0.4.0";
      sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" = overridableMkRustCrate ({ profileName, profile }: {
    inherit release profile;
    name = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo {
      name = "winapi-x86_64-pc-windows-gnu";
      version = "0.4.0";
      sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f";
    };
    features = builtins.concatLists [
    ];
    dependencies = {
    };
    devDependencies = {
    };
    buildDependencies = {
    };
  });
  
}

