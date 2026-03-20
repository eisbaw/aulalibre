#!/usr/bin/env bash

# APK Recursive Extraction Script
# Extracts APK files recursively, preserving hierarchy and origin information
# Handles both regular APK and XAPK (split APK bundle) formats

set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Configuration
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_FILE="${SCRIPT_DIR}/apk_extraction.log"
readonly MANIFEST_FILE="${SCRIPT_DIR}/extraction_manifest.txt"
readonly MAX_DEPTH=10  # Prevent infinite recursion

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Logging functions — write to stderr so stdout stays clean for data flow
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*" >> "$LOG_FILE"
    echo -e "${BLUE}[INFO]${NC} $*" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*" >> "$LOG_FILE"
    echo -e "${GREEN}[SUCCESS]${NC} $*" >&2
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*" >> "$LOG_FILE"
    echo -e "${YELLOW}[WARNING]${NC} $*" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >> "$LOG_FILE"
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

# Initialize log file
init_logging() {
    echo "APK Extraction Log - $(date)" > "$LOG_FILE"
    echo "===========================================" >> "$LOG_FILE"
    echo "APK Extraction Manifest - $(date)" > "$MANIFEST_FILE"
    echo "=========================================" >> "$MANIFEST_FILE"
}

# Check if file is an APK
is_apk_file() {
    local file="$1"

    # Check extension
    if [[ "${file,,}" =~ \.(apk|xapk)$ ]]; then
        return 0
    fi

    # Check magic number (ZIP signature)
    if file "$file" | grep -q "archive data\|ZIP\|Zip"; then
        # Additional check for APK-specific content
        if unzip -l "$file" 2>/dev/null | grep -q "AndroidManifest.xml\|classes.dex\|META-INF"; then
            return 0
        fi
    fi

    return 1
}

# Extract single APK file
extract_apk() {
    local apk_file="$1"
    local depth="$2"
    local parent_path="${3:-}"

    # Check recursion depth
    if (( depth > MAX_DEPTH )); then
        log_error "Maximum recursion depth ($MAX_DEPTH) reached for: $apk_file"
        return 1
    fi

    # Create extraction directory
    local extract_dir="${apk_file}.extracted"

    log_info "Extracting APK: $apk_file (depth: $depth)"
    log_info "Target directory: $extract_dir"

    # Create directory and extract
    if ! mkdir -p "$extract_dir"; then
        log_error "Failed to create directory: $extract_dir"
        return 1
    fi

    # Extract with error handling (-o to overwrite without prompting)
    local unzip_output
    if ! unzip_output=$(unzip -o -q "$apk_file" -d "$extract_dir" 2>&1); then
        log_error "Failed to extract APK: $apk_file"
        log_error "unzip output: $unzip_output"
        return 1
    fi

    # Log extraction to manifest
    {
        echo "APK: $(realpath "$apk_file")"
        echo "  Extracted to: $(realpath "$extract_dir")"
        echo "  Depth: $depth"
        echo "  Parent: $parent_path"
        echo "  Size: $(du -h "$apk_file" | cut -f1)"
        echo "  Timestamp: $(date)"
        echo "  ---"
    } >> "$MANIFEST_FILE"

    log_success "Successfully extracted: $apk_file"

    # Get basic info about extracted content
    local file_count
    file_count=$(find "$extract_dir" -type f | wc -l)
    log_info "  Files extracted: $file_count"

    # Look for interesting files
    local dex_files
    dex_files=$(find "$extract_dir" -name "*.dex" | wc -l)
    if (( dex_files > 0 )); then
        log_info "  DEX files found: $dex_files"
    fi

    local so_files
    so_files=$(find "$extract_dir" -name "*.so" | wc -l)
    if (( so_files > 0 )); then
        log_info "  Native libraries (.so): $so_files"
    fi

    # Check for AndroidManifest.xml
    if [[ -f "$extract_dir/AndroidManifest.xml" ]]; then
        log_info "  AndroidManifest.xml found"
    fi

    # Recursively search for more APK files
    log_info "Scanning for nested APK files in: $extract_dir"

    local nested_apks=()
    while IFS= read -r -d '' nested_apk; do
        nested_apks+=("$nested_apk")
    done < <(find "$extract_dir" -type f -print0 | while IFS= read -r -d '' file; do
        if is_apk_file "$file"; then
            printf '%s\0' "$file"
        fi
    done)

    if (( ${#nested_apks[@]} > 0 )); then
        log_info "Found ${#nested_apks[@]} nested APK files"
        for nested_apk in "${nested_apks[@]}"; do
            log_info "  Nested APK: $nested_apk"
            extract_apk "$nested_apk" $((depth + 1)) "$apk_file"
        done
    else
        log_info "No nested APK files found"
    fi

    return 0
}

# Find all APK files in current directory
find_apk_files() {
    local apk_files=()

    log_info "Scanning for APK files in current directory..."

    for file in *.apk *.xapk; do
        if [[ -f "$file" ]] && is_apk_file "$file"; then
            apk_files+=("$file")
            log_info "Found APK: $file"
        fi
    done

    # Also check for files that might be APKs without .apk extension
    for file in *; do
        if [[ -f "$file" ]] && [[ ! "$file" =~ \.(apk|xapk)$ ]] && is_apk_file "$file"; then
            apk_files+=("$file")
            log_info "Found APK (no .apk extension): $file"
        fi
    done

    printf '%s\n' "${apk_files[@]}"
}

# Generate per-file manifest with metadata (type, size, path)
generate_file_manifest() {
    log_info "Generating per-file manifest..."

    {
        echo ""
        echo "Per-File Manifest"
        echo "========================================="
        echo "FORMAT: size_bytes | file_type | path"
        echo ""
        find . -path "*.extracted/*" -type f -print0 | while IFS= read -r -d '' f; do
            local fsize ftype
            fsize=$(stat --printf='%s' "$f" 2>/dev/null || echo "0")
            ftype=$(file -b --mime-type "$f" 2>/dev/null || echo "unknown")
            printf '%s | %s | %s\n' "$fsize" "$ftype" "$f"
        done | sort -t'|' -k3
    } >> "$MANIFEST_FILE"

    log_success "Per-file manifest appended to: $MANIFEST_FILE"
}

# Generate extraction summary with file type counts
generate_summary() {
    log_info "Generating extraction summary..."

    local summary_file="${SCRIPT_DIR}/extraction_summary.txt"
    {
        echo "APK Extraction Summary - $(date)"
        echo "====================================="
        echo

        echo "Original APK files processed:"
        grep -c "^  Depth: 0" "$MANIFEST_FILE" || echo "0"
        echo

        echo "Total APK files extracted (including nested):"
        grep -c "^APK:" "$MANIFEST_FILE" || echo "0"
        echo

        echo "Extraction directories created:"
        find . -type d -name "*.extracted" | wc -l
        echo

        echo "Total files extracted:"
        find . -path "*.extracted/*" -type f | wc -l
        echo

        echo "Total DEX files found:"
        find . -name "*.dex" -path "*.extracted/*" | wc -l
        echo

        echo "Total native libraries found:"
        find . -name "*.so" -path "*.extracted/*" | wc -l
        echo

        echo "AndroidManifest.xml files:"
        find . -name "AndroidManifest.xml" -path "*.extracted/*" | wc -l
        echo

        echo "Disk space used by extractions:"
        du -sh *.extracted 2>/dev/null || echo "None"
        echo

        echo "File counts by extension:"
        echo "---"
        find . -path "*.extracted/*" -type f | sed 's/.*\.//' | sort | uniq -c | sort -rn
        echo

        echo "File counts by MIME type:"
        echo "---"
        find . -path "*.extracted/*" -type f -print0 | xargs -0 file --mime-type -b 2>/dev/null | sort | uniq -c | sort -rn

    } > "$summary_file"

    log_success "Summary written to: $summary_file"
    cat "$summary_file" >&2
}

# Main function
main() {
    log_info "Starting APK recursive extraction"
    log_info "Script directory: $SCRIPT_DIR"
    log_info "Maximum recursion depth: $MAX_DEPTH"

    # Initialize logging
    init_logging

    # Find APK files
    local apk_files
    readarray -t apk_files < <(find_apk_files)

    if (( ${#apk_files[@]} == 0 )); then
        log_error "No APK files found in current directory"
        exit 1
    fi

    log_info "Found ${#apk_files[@]} APK file(s) to process"

    # Process each APK file
    local success_count=0
    local total_count=${#apk_files[@]}

    for apk_file in "${apk_files[@]}"; do
        log_info "Processing APK file: $apk_file"

        if extract_apk "$apk_file" 0; then
            ((success_count++)) || true
        else
            log_error "Failed to process: $apk_file"
        fi

        echo "----------------------------------------"
    done

    # Generate per-file manifest and summary
    generate_file_manifest
    generate_summary

    # Final status
    log_info "Extraction complete!"
    log_info "Successfully processed: $success_count/$total_count APK files"

    if (( success_count == total_count )); then
        log_success "All APK files extracted successfully!"
        exit 0
    else
        log_warning "Some APK files failed to extract. Check log for details."
        exit 1
    fi
}

# Check dependencies
check_dependencies() {
    local deps=("unzip" "find" "file" "du")
    local missing=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done

    if (( ${#missing[@]} > 0 )); then
        log_error "Missing dependencies: ${missing[*]}"
        log_error "Please run: nix-shell --run '$0'"
        exit 1
    fi
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    check_dependencies
    main "$@"
fi
