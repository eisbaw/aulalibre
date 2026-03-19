{ pkgs ? import <nixpkgs> {} }:

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
    jadx          # DEX to Java decompiler (GUI and CLI)
    apktool       # APK reverse engineering tool

    # Java decompilation ecosystem
    jdk           # Java Development Kit for running Java tools

    # Binary analysis tools
    binwalk       # Firmware analysis tool, extracts embedded files
    binutils      # Binary utilities (includes strings, objdump, readelf)
    hexdump       # Hex dump utility for binary inspection

    # Disassemblers and reverse engineering
    radare2       # Advanced reverse engineering framework
    ghidra        # NSA's reverse engineering suite

    # .NET decompilation (if needed)
    mono          # .NET runtime for Linux
    ilspycmd      # ILSpy command-line .NET decompiler

    # Network and protocol analysis
    wireshark     # Network protocol analyzer (for runtime analysis)

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
  ];

  shellHook = ''
    export JADX_OPTS="-Xmx4g"
    export JAVA_OPTS="-Xmx4g"
    mkdir -p extracted analysis decompiled reports
  '';
}
