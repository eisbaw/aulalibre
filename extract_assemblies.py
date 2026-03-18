#!/usr/bin/env python3
"""Extract .NET assemblies from Xamarin Android blob (libassemblies.*.blob.so)

Format:
- XABA header at offset 0x4000 in the ELF
- Assembly entries with XALZ magic (LZ4 compressed)
- XALZ header: 4 bytes magic, 4 bytes index, 4 bytes uncompressed_size
- Followed by LZ4 block compressed data

Since we don't have lz4 Python module, we extract the raw compressed blocks
and try to decompress with available tools, or just extract strings from the
compressed data directly.
"""

import struct
import os
import sys

def lz4_decompress_block(compressed_data, uncompressed_size):
    """Manual LZ4 block decompression.

    LZ4 block format:
    - Token byte: high nibble = literal length, low nibble = match length
    - If literal length == 15, read additional bytes until < 255
    - Literal data
    - Offset (2 bytes, little endian) - not present if last sequence
    - If match length == 15, read additional bytes until < 255
    - Match length += 4 (minimum match)
    """
    output = bytearray()
    pos = 0

    while pos < len(compressed_data):
        # Token
        token = compressed_data[pos]
        pos += 1

        # Literal length
        literal_length = (token >> 4) & 0x0F
        if literal_length == 15:
            while pos < len(compressed_data):
                extra = compressed_data[pos]
                pos += 1
                literal_length += extra
                if extra != 255:
                    break

        # Copy literals
        if pos + literal_length > len(compressed_data):
            # Copy what we can
            output.extend(compressed_data[pos:])
            break
        output.extend(compressed_data[pos:pos + literal_length])
        pos += literal_length

        if len(output) >= uncompressed_size:
            break

        if pos + 2 > len(compressed_data):
            break

        # Match offset
        offset = struct.unpack('<H', compressed_data[pos:pos + 2])[0]
        pos += 2

        if offset == 0:
            break

        # Match length
        match_length = token & 0x0F
        if match_length == 15:
            while pos < len(compressed_data):
                extra = compressed_data[pos]
                pos += 1
                match_length += extra
                if extra != 255:
                    break
        match_length += 4  # Minimum match length

        # Copy match
        match_pos = len(output) - offset
        if match_pos < 0:
            break
        for i in range(match_length):
            if match_pos + (i % offset) < len(output):
                output.append(output[match_pos + (i % offset)])
            else:
                break

        if len(output) >= uncompressed_size:
            break

    return bytes(output[:uncompressed_size])


def main():
    blob_path = 'com.netcompany.aulanativeprivate.xapk.extracted/config.x86_64.apk.extracted/lib/x86_64/libassemblies.x86_64.blob.so'
    output_dir = 'extracted_assemblies'

    os.makedirs(output_dir, exist_ok=True)

    with open(blob_path, 'rb') as f:
        data = f.read()

    # Find all XALZ headers
    magic = b'XALZ'
    offsets = []
    pos = 0
    while True:
        pos = data.find(magic, pos)
        if pos == -1:
            break
        offsets.append(pos)
        pos += 1

    print(f"Found {len(offsets)} XALZ compressed assemblies")

    # Try to find assembly name mapping
    # In newer Xamarin, names are in the assembly store hash table
    # For now, we'll extract by index and try to identify by content

    target_names = ['AulaNative', 'IdentityModel']

    for i, off in enumerate(offsets):
        idx = struct.unpack('<I', data[off+4:off+8])[0]
        uncomp_size = struct.unpack('<I', data[off+8:off+12])[0]

        # Determine compressed data boundaries
        next_off = offsets[i+1] if i+1 < len(offsets) else len(data)
        comp_data = data[off+12:next_off]

        # Try to decompress
        try:
            decompressed = lz4_decompress_block(comp_data, uncomp_size)
        except Exception as e:
            continue

        # Check if this is a valid .NET assembly (PE header)
        if len(decompressed) < 2:
            continue

        # Look for assembly name in the decompressed data
        name = None
        for target in target_names + ['AulaNative.dll', 'AulaNative.Droid.dll', 'AulaNative.Droid.Private.dll',
                                       'IdentityModel.dll', 'IdentityModel.OidcClient.dll',
                                       'Newtonsoft.Json.dll', 'AutoMapper.dll']:
            if target.encode() in decompressed:
                name = target.replace('.dll', '')
                break

        if name and any(t in (name or '') for t in target_names):
            out_path = os.path.join(output_dir, f"assembly_{idx}_{name}.dll")
            with open(out_path, 'wb') as f:
                f.write(decompressed)
            print(f"  Extracted: {out_path} ({len(decompressed)} bytes, expected {uncomp_size})")

            # Quick strings extraction
            strings_path = os.path.join(output_dir, f"assembly_{idx}_{name}.strings.txt")
            strings = []
            current = []
            for byte in decompressed:
                if 32 <= byte < 127:
                    current.append(chr(byte))
                else:
                    if len(current) >= 6:
                        strings.append(''.join(current))
                    current = []
            if len(current) >= 6:
                strings.append(''.join(current))

            with open(strings_path, 'w') as f:
                f.write('\n'.join(strings))
            print(f"  Strings: {strings_path} ({len(strings)} strings)")

    # Also extract ALL assemblies that contain interesting patterns
    print("\nScanning all assemblies for API-related content...")
    api_findings = []

    for i, off in enumerate(offsets):
        idx = struct.unpack('<I', data[off+4:off+8])[0]
        uncomp_size = struct.unpack('<I', data[off+8:off+12])[0]

        next_off = offsets[i+1] if i+1 < len(offsets) else len(data)
        comp_data = data[off+12:next_off]

        try:
            decompressed = lz4_decompress_block(comp_data, uncomp_size)
        except:
            continue

        # Check for API-relevant content
        content_lower = decompressed.lower()
        has_api = (b'/api/' in content_lower or
                   b'aula.dk' in content_lower or
                   b'ncaula.com' in content_lower or
                   b'baseurl' in content_lower or
                   b'baseaddress' in content_lower or
                   b'httpmethod' in content_lower or
                   b'getasync' in content_lower or
                   b'postasync' in content_lower)

        if has_api:
            # Extract strings from this assembly
            strings = []
            current = []
            for byte in decompressed:
                if 32 <= byte < 127:
                    current.append(chr(byte))
                else:
                    if len(current) >= 4:
                        strings.append(''.join(current))
                    current = []

            # Filter for API-relevant strings
            api_strings = [s for s in strings if any(kw in s.lower() for kw in
                ['api', 'http', 'url', 'endpoint', 'aula', 'ncaula', 'token', 'auth',
                 'service', 'client', 'request', 'response', 'header', 'bearer',
                 'calendar', 'message', 'notification', 'profile', 'post', 'album',
                 'media', 'document', 'group', 'presence', 'absence', 'vacation',
                 'comego', 'lesson', 'event', 'meeting', 'consent', 'institution',
                 'schedule', 'contact', 'search'])]

            if api_strings:
                # Find assembly name
                name = f"unknown_{idx}"
                for s in strings:
                    if s.endswith('.dll') and len(s) < 100:
                        name = s.replace('.dll', '')
                        break

                api_findings.append((idx, name, api_strings))

                # Save full assembly for analysis
                out_path = os.path.join(output_dir, f"assembly_{idx}_{name.replace('/', '_')}.dll")
                with open(out_path, 'wb') as f:
                    f.write(decompressed)

    # Report findings
    print(f"\nFound {len(api_findings)} assemblies with API-related content")

    # Write API findings report
    report_path = os.path.join(output_dir, 'api_findings.txt')
    with open(report_path, 'w') as f:
        for idx, name, strings in api_findings:
            f.write(f"\n=== Assembly {idx}: {name} ===\n")
            for s in sorted(set(strings)):
                f.write(f"  {s}\n")

    print(f"Report written to: {report_path}")


if __name__ == '__main__':
    main()
