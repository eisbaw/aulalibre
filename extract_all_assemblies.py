#!/usr/bin/env python3
"""Extract ALL .NET assemblies from Xamarin Android blob (libassemblies.*.blob.so).

Format:
- ELF shared library with a 'payload' section starting at offset 0x4000
- XABA header: magic(4) + version(4) + local_entry_count(4) + global_entry_count(4) + store_id(4)
- Followed by an index, then XALZ-compressed assembly entries
- XALZ entry: magic 'XALZ'(4) + descriptor_index(4) + uncompressed_size(4) + LZ4 compressed data

Requires: python3 with lz4 module (available via shell.nix)
"""

import struct
import os
import sys

try:
    import lz4.block
    HAS_LZ4 = True
except ImportError:
    HAS_LZ4 = False
    print("WARNING: lz4 module not available, using manual decompressor (slower, less reliable)")


def lz4_decompress_block_manual(compressed_data, uncompressed_size):
    """Manual LZ4 block decompression fallback."""
    output = bytearray()
    pos = 0

    while pos < len(compressed_data) and len(output) < uncompressed_size:
        token = compressed_data[pos]
        pos += 1

        literal_length = (token >> 4) & 0x0F
        if literal_length == 15:
            while pos < len(compressed_data):
                extra = compressed_data[pos]
                pos += 1
                literal_length += extra
                if extra != 255:
                    break

        if pos + literal_length > len(compressed_data):
            output.extend(compressed_data[pos:])
            break
        output.extend(compressed_data[pos:pos + literal_length])
        pos += literal_length

        if len(output) >= uncompressed_size:
            break

        if pos + 2 > len(compressed_data):
            break

        offset = struct.unpack('<H', compressed_data[pos:pos + 2])[0]
        pos += 2

        if offset == 0:
            break

        match_length = token & 0x0F
        if match_length == 15:
            while pos < len(compressed_data):
                extra = compressed_data[pos]
                pos += 1
                match_length += extra
                if extra != 255:
                    break
        match_length += 4

        match_pos = len(output) - offset
        if match_pos < 0:
            break
        for i in range(match_length):
            if match_pos + (i % offset) < len(output):
                output.append(output[match_pos + (i % offset)])
            else:
                break

    return bytes(output[:uncompressed_size])


def decompress_xalz(compressed_data, uncompressed_size):
    """Decompress LZ4 block data from XALZ entry."""
    if HAS_LZ4:
        try:
            return lz4.block.decompress(compressed_data, uncompressed_size=uncompressed_size)
        except Exception:
            return lz4_decompress_block_manual(compressed_data, uncompressed_size)
    else:
        return lz4_decompress_block_manual(compressed_data, uncompressed_size)


def guess_assembly_name(decompressed_data):
    """Extract the assembly name from PE metadata.

    .NET assemblies reference their own module name as a .dll string early in
    the metadata. The first occurrence of a name ending in '.dll' (that is not
    'mscoree.dll' which is the runtime import) is reliably the assembly's own name.
    """
    search_pos = 0
    while True:
        p = decompressed_data.find(b".dll", search_pos)
        if p == -1:
            break

        # Walk backwards to find start of the name
        start = p - 1
        while start >= 0 and 32 <= decompressed_data[start] < 127:
            start -= 1
        start += 1

        try:
            name = decompressed_data[start:p + 4].decode("ascii")
        except (UnicodeDecodeError, ValueError):
            search_pos = p + 1
            continue

        # Skip mscoree.dll (runtime import present in all assemblies)
        # Skip very short or suspicious names
        if (len(name) > 5
                and name.endswith(".dll")
                and name != "mscoree.dll"
                and not name.startswith(".")
                and " " not in name):
            return name[:-4]  # Strip .dll extension

        search_pos = p + 1

    return None


def categorize_assembly(name):
    """Categorize an assembly by its name."""
    if name is None:
        return "unknown"

    name_lower = name.lower()

    # Aula-specific
    if "aula" in name_lower:
        return "aula-app"

    # Netcompany
    if "netcompany" in name_lower:
        return "aula-app"

    # Key third-party libs we want to decompile
    third_party_decompile = [
        "identitymodel", "plugin.", "monkeycache", "sqlite-net",
        "sqlitepcl", "sqlite",
        "automapper", "newtonsoft", "signalr", "polly", "refit",
        "akavache", "fusillade", "splat", "reactiveui",
        "xam.plugin", "ffimageloading", "prism", "dryioc",
        "humanizer", "nito", "pcl", "connectivity",
        "lottie", "photoview", "bottombar", "bottomnavigationbar",
        "carouselview", "glide", "picasso",
    ]
    for lib in third_party_decompile:
        if lib in name_lower:
            return "third-party-key"

    # Firebase / Google
    if "firebase" in name_lower or "google" in name_lower:
        return "google-firebase"

    # Xamarin-specific
    if "xamarin" in name_lower:
        return "xamarin"

    # AndroidX
    if name_lower.startswith("androidx."):
        return "androidx"

    # Microsoft / .NET framework
    if (name_lower.startswith("microsoft.")
            or name_lower.startswith("system.")
            or name_lower in ("mscorlib", "netstandard", "mscoree")):
        return "framework"

    if name_lower.startswith("mono."):
        return "mono-runtime"

    # Java/Android interop
    if (name_lower.startswith("java.")
            or name_lower.startswith("javax.")
            or name_lower.startswith("android.")
            or name_lower.startswith("kotlin")):
        return "java-interop"

    # FxResources are framework satellite assemblies
    if name_lower.startswith("fxresources."):
        return "framework"

    return "third-party-other"


def should_decompile(name, category):
    """Determine if this assembly should be decompiled with ILSpy."""
    if category in ("aula-app", "third-party-key"):
        return True
    # Also decompile some specific interesting framework assemblies
    if name and any(x in name.lower() for x in ["identity", "auth"]):
        return True
    return False


def main():
    blob_path = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "com.netcompany.aulanativeprivate.xapk.extracted",
        "config.x86_64.apk.extracted",
        "lib", "x86_64",
        "libassemblies.x86_64.blob.so"
    )
    output_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "extracted_assemblies_all")
    inventory_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "assembly_inventory.md")

    os.makedirs(output_dir, exist_ok=True)

    print(f"Reading blob: {blob_path}")
    with open(blob_path, "rb") as f:
        data = f.read()
    print(f"Blob size: {len(data):,} bytes")

    # Parse XABA header at offset 0x4000
    payload_offset = 0x4000
    xaba_magic = data[payload_offset:payload_offset + 4]
    if xaba_magic != b"XABA":
        print(f"ERROR: Expected XABA magic at 0x4000, got {xaba_magic}")
        sys.exit(1)

    version = struct.unpack_from("<I", data, payload_offset + 4)[0]
    local_count = struct.unpack_from("<I", data, payload_offset + 8)[0]
    global_count = struct.unpack_from("<I", data, payload_offset + 12)[0]
    store_id = struct.unpack_from("<I", data, payload_offset + 16)[0]

    print(f"XABA header: version={hex(version)}, local_entries={local_count}, "
          f"global_entries={global_count}, store_id={store_id}")

    # Find all XALZ entries
    xalz_magic = b"XALZ"
    offsets = []
    pos = 0
    while True:
        pos = data.find(xalz_magic, pos)
        if pos == -1:
            break
        offsets.append(pos)
        pos += 1

    print(f"Found {len(offsets)} XALZ compressed assembly entries")

    # Extract all assemblies
    assemblies = []
    failures = 0

    for i, off in enumerate(offsets):
        idx = struct.unpack("<I", data[off + 4:off + 8])[0]
        uncomp_size = struct.unpack("<I", data[off + 8:off + 12])[0]

        # Compressed data runs from offset+12 to the next XALZ entry
        next_off = offsets[i + 1] if i + 1 < len(offsets) else len(data)
        comp_data = data[off + 12:next_off]

        try:
            decompressed = decompress_xalz(comp_data, uncomp_size)
        except Exception as e:
            print(f"  FAIL [{i}/{len(offsets)}] idx={idx}: decompress error: {e}")
            failures += 1
            continue

        # Verify it's a PE file
        is_pe = len(decompressed) >= 2 and decompressed[0:2] == b"MZ"
        if not is_pe:
            print(f"  WARN [{i}/{len(offsets)}] idx={idx}: not a PE file "
                  f"(first bytes: {decompressed[:4].hex()})")

        # Try to determine assembly name
        name = guess_assembly_name(decompressed)
        category = categorize_assembly(name)

        if name:
            safe_name = name.replace("/", "_").replace("\\", "_")
            filename = f"{safe_name}.dll"
        else:
            filename = f"unknown_{idx:04d}.dll"
            name = f"unknown_{idx:04d}"

        out_path = os.path.join(output_dir, filename)

        # Handle duplicate names (some assemblies might share names in different configs)
        if os.path.exists(out_path):
            with open(out_path, "rb") as existing:
                if existing.read() == decompressed:
                    # Identical content, skip writing but still record
                    assemblies.append({
                        "index": idx,
                        "name": name,
                        "size": len(decompressed),
                        "compressed_size": len(comp_data),
                        "is_pe": is_pe,
                        "category": category,
                        "filename": filename,
                        "duplicate": True,
                    })
                    continue
            # Different content, add suffix
            base, ext = os.path.splitext(filename)
            filename = f"{base}_idx{idx}{ext}"
            out_path = os.path.join(output_dir, filename)

        with open(out_path, "wb") as f:
            f.write(decompressed)

        assemblies.append({
            "index": idx,
            "name": name,
            "size": len(decompressed),
            "compressed_size": len(comp_data),
            "is_pe": is_pe,
            "category": category,
            "filename": filename,
            "duplicate": False,
        })

        if (i + 1) % 50 == 0:
            print(f"  Extracted {i + 1}/{len(offsets)}...")

    print(f"\nExtraction complete: {len(assemblies)} assemblies, {failures} failures")

    # Unique names (non-duplicate entries)
    unique_names = set()
    for a in assemblies:
        unique_names.add(a["name"])
    print(f"Unique assembly names: {len(unique_names)}")

    # Duplicate count
    dup_count = sum(1 for a in assemblies if a["duplicate"])
    print(f"Duplicate entries (identical content, not re-written): {dup_count}")

    # Print category summary
    cats = {}
    for a in assemblies:
        if not a["duplicate"]:
            cats[a["category"]] = cats.get(a["category"], 0) + 1
    print("\nCategory breakdown (unique only):")
    for cat, count in sorted(cats.items()):
        print(f"  {cat}: {count}")

    # Generate inventory markdown -- only unique assemblies
    unique_assemblies = []
    seen_names = set()
    for a in assemblies:
        if a["name"] not in seen_names:
            seen_names.add(a["name"])
            unique_assemblies.append(a)

    print(f"\nWriting inventory to {inventory_path}")
    with open(inventory_path, "w") as f:
        f.write("# Assembly Inventory\n\n")
        f.write(f"Extracted from `libassemblies.x86_64.blob.so` ({len(data):,} bytes)\n\n")
        f.write(f"- **Total XALZ entries**: {len(offsets)}\n")
        f.write(f"- **Successfully extracted**: {len(assemblies)}\n")
        f.write(f"- **Unique assemblies**: {len(unique_assemblies)}\n")
        f.write(f"- **Duplicate entries**: {dup_count}\n")
        f.write(f"- **Extraction failures**: {failures}\n\n")

        f.write("## Category Summary\n\n")
        f.write("| Category | Count |\n")
        f.write("|----------|-------|\n")
        for cat, count in sorted(cats.items()):
            f.write(f"| {cat} | {count} |\n")
        f.write("\n")

        # Group by category
        by_category = {}
        for a in unique_assemblies:
            by_category.setdefault(a["category"], []).append(a)

        # Define display order
        cat_order = [
            "aula-app", "third-party-key", "third-party-other",
            "google-firebase", "xamarin", "androidx", "framework",
            "mono-runtime", "java-interop", "unknown"
        ]
        for c in by_category:
            if c not in cat_order:
                cat_order.append(c)

        for cat in cat_order:
            if cat not in by_category:
                continue
            items = sorted(by_category[cat], key=lambda x: x["name"])
            f.write(f"## {cat}\n\n")
            f.write("| Name | Size | Compressed | Decompile? |\n")
            f.write("|------|------|-----------|------------|\n")
            for a in items:
                size_kb = a["size"] / 1024
                comp_kb = a["compressed_size"] / 1024
                decompile = "YES" if should_decompile(a["name"], a["category"]) else "no"
                f.write(f"| {a['name']} | {size_kb:.1f} KB | {comp_kb:.1f} KB | {decompile} |\n")
            f.write("\n")

    # List assemblies to decompile
    to_decompile = [a for a in unique_assemblies
                    if should_decompile(a["name"], a["category"]) and a["is_pe"]]

    print(f"\nAssemblies to decompile with ILSpy: {len(to_decompile)}")
    for a in sorted(to_decompile, key=lambda x: x["name"]):
        print(f"  {a['name']} ({a['size'] / 1024:.1f} KB)")

    # Write decompile list for the next step
    decompile_list_path = os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "assemblies_to_decompile.txt"
    )
    with open(decompile_list_path, "w") as f:
        for a in sorted(to_decompile, key=lambda x: x["name"]):
            f.write(f"{a['filename']}\n")
    print(f"Decompile list written to: {decompile_list_path}")


if __name__ == "__main__":
    main()
