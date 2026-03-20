#!/usr/bin/env bash
# Decompile key .NET assemblies with ILSpy
# Run inside nix-shell: nix-shell --run './decompile_assemblies.sh'

set -euo pipefail

ASSEMBLIES_DIR="$(dirname "$0")/extracted_assemblies_all"
DECOMPILED_DIR="$(dirname "$0")/decompiled_csharp"
LIST_FILE="$(dirname "$0")/assemblies_to_decompile.txt"

mkdir -p "$DECOMPILED_DIR"

total=0
success=0
skipped=0
failed=0

while IFS= read -r dll_file; do
    [ -z "$dll_file" ] && continue
    total=$((total + 1))

    assembly_name="${dll_file%.dll}"
    # Strip _idx suffix if present
    assembly_name=$(echo "$assembly_name" | sed 's/_idx[0-9]*$//')

    src_path="$ASSEMBLIES_DIR/$dll_file"
    out_dir="$DECOMPILED_DIR/$assembly_name"

    if [ -d "$out_dir" ] && [ "$(ls -A "$out_dir" 2>/dev/null)" ]; then
        echo "SKIP: $assembly_name (already decompiled)"
        skipped=$((skipped + 1))
        continue
    fi

    if [ ! -f "$src_path" ]; then
        echo "MISSING: $src_path"
        failed=$((failed + 1))
        continue
    fi

    echo "Decompiling: $assembly_name..."
    mkdir -p "$out_dir"

    if ilspycmd -p -o "$out_dir" "$src_path" 2>/dev/null; then
        file_count=$(find "$out_dir" -name '*.cs' | wc -l)
        echo "  OK: $file_count .cs files"
        success=$((success + 1))
    else
        echo "  FAIL: ilspycmd returned error"
        failed=$((failed + 1))
        rmdir "$out_dir" 2>/dev/null || true
    fi
done < "$LIST_FILE"

echo ""
echo "Decompilation summary:"
echo "  Total: $total"
echo "  Success: $success"
echo "  Skipped: $skipped"
echo "  Failed: $failed"
