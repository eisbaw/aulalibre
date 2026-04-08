# default.nix — Build the Aula Rust workspace (aula-cli, aula-fuse).
#
# This is a pure build derivation producing installable binaries.
# It is intentionally separate from shell.nix:
#
#   shell.nix  = interactive dev environment (tools, compilers, RE utilities)
#   default.nix = reproducible build artifact (nix-build / nix-env -if .)
#
# shell.nix does NOT import default.nix — they share the same nixpkgs pin
# but serve different purposes. Mixing them would either bloat the dev shell
# with build-time closure, or break the build with dev-only impurities.

{
  pkgs ? (import (builtins.fetchTarball {
           url = "https://github.com/nixos/nixpkgs/tarball/25.11";
           sha256 = "1zn1lsafn62sz6azx6j735fh4vwwghj8cc9x91g5sx2nrg23ap9k";
         }) {})
}:

let
  cargoToml = builtins.fromTOML (builtins.readFile ./aula/aula-cli/Cargo.toml);
in

pkgs.rustPlatform.buildRustPackage {
  pname = "aula";
  version = cargoToml.package.version;

  src = ./aula;

  cargoHash = "sha256-5EpZ3CwzMi6kANuJT+5fLZA7ofmlz8m1Ilam0QYj2bw=";

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    openssl
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    fuse3
  ];

  # Build all workspace members
  cargoBuildFlags = [ "--workspace" ];

  # Test all workspace members
  cargoTestFlags = [ "--workspace" ];

  meta = with pkgs.lib; {
    description = "CLI and FUSE filesystem for the Aula school platform";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
