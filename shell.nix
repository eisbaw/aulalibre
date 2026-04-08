# shell.nix — Interactive development environment for the Aula project.
#
# Provides compilers, RE tools, analysis utilities, and dev tooling.
# This is NOT a build derivation — use default.nix for that:
#
#   shell.nix  = interactive dev environment (nix-shell)
#   default.nix = reproducible build artifact (nix-build)
#
# These two files are intentionally independent. shell.nix does not import
# default.nix because the dev shell needs tools (jadx, ghidra, wireshark, …)
# that have nothing to do with building the Rust binaries, and the build
# derivation must stay pure and minimal.

{
  pkgs ? (import (builtins.fetchTarball {
           url = "https://github.com/nixos/nixpkgs/tarball/25.11";
           sha256 = "1zn1lsafn62sz6azx6j735fh4vwwghj8cc9x91g5sx2nrg23ap9k";
         }) {})
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Basic file and archive manipulation
    unzip         # Extract APK/XAPK files (both are ZIP archives)
    zip           # Create archives if needed
    file          # Identify file types using magic numbers
    tree          # Display directory structures
    findutils     # Find files recursively (find, xargs)

    # Android-specific tools
    android-tools # adb, aapt, and other Android SDK tools
    apktool       # APK reverse engineering tool

    # Java decompilation ecosystem
    jdk           # Java Development Kit for running Java tools

    # Binary analysis tools
    binwalk       # Firmware analysis tool, extracts embedded files
    binutils      # Binary utilities (includes strings, objdump, readelf)
    hexdump       # Hex dump utility for binary inspection

    # Disassemblers and reverse engineering
    radare2       # Advanced reverse engineering framework

    # Text processing and analysis
    ripgrep       # Fast text search tool
    jq            # JSON processor for analyzing config files
    xmlstarlet    # XML processing toolkit (for AndroidManifest.xml)

    # Scripting and automation
    bash          # Bash shell for our extraction script
    coreutils     # Core Unix utilities (ls, cat, etc.)

    # Documentation and reporting
    pandoc        # Convert between document formats
    graphviz      # Generate diagrams for architecture docs

    # Archive and compression tools
    p7zip         # Handle various archive formats
    gzip          # Gzip compression/decompression

    # Rust toolchain
    rustc         # Rust compiler
    cargo         # Rust package manager and build tool
    clippy        # Rust linter
    rustfmt       # Rust code formatter
    rust-analyzer # Rust language server
    pkg-config    # Build dependency for native crates
    openssl.dev   # OpenSSL headers for reqwest/native-tls

    # Development tools
    just          # Command runner for project recipes
    git           # Version control for tracking analysis progress
    curl          # Download additional resources if needed
    wget          # Alternative download tool

    # Python environment (without pip install)
    python3       # Python interpreter for custom analysis scripts
    python3Packages.requests  # HTTP library
    python3Packages.lxml      # XML processing
    python3Packages.beautifulsoup4  # HTML/XML parsing
    python3Packages.lz4             # LZ4 compression (for Xamarin blob extraction)

    # Forensics and security tools
    sleuthkit     # Digital forensics toolkit
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    # macOS: stubs provide FUSE headers for compilation; macFUSE or fuse-t needed at runtime
    macfuse-stubs # FUSE pkg-config headers for compiling fuser crate
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    # Linux-only packages (fail to build or unavailable on Darwin)
    fuse3         # FUSE3 library for aula-fuse
    jadx          # DEX to Java decompiler — build broken on Darwin
    ghidra        # NSA's reverse engineering suite — Linux-only deps
    wireshark     # Network protocol analyzer — build issues on Darwin
    mono          # .NET runtime for Linux
    ilspycmd      # ILSpy command-line .NET decompiler
  ];

  shellHook = ''
    export JADX_OPTS="-Xmx4g"
    export JAVA_OPTS="-Xmx4g"
    mkdir -p extracted analysis decompiled reports
  '';
}
