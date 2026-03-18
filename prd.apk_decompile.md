# Aula App Decompilation Project Requirements Document

## Project Overview
This project aims to decompile and analyze the Aula Android APK to understand its structure, business logic, and architecture. The target APK is `com.netcompany.aulanativeprivate` - the Aula school communication platform developed by Netcompany A/S. Aula is Denmark's primary digital communication platform between schools, parents, and students.

## Technical Stack
- **Environment**: Nix shell environment with declarative tooling
- **Scripting**: Bash for automation
- **Decompilation Tools**: Multiple tools for different file types (DEX, .NET, ELF)
- **Analysis**: File magic, binwalk, and specialized decompilers

## Project Milestones

### Milestone 1: Recursive APK Extraction
**Goal**: Extract all APK files recursively, preserving hierarchy and origin information.

**Deliverables**:
- `apk_extract.sh` - Bash script for recursive APK extraction
- Extracted APK structure in `*.apk.extracted/` directories

**Acceptance Criteria**:
- All nested APK files are discovered and extracted
- Directory structure preserves origin and hierarchy
- Script handles edge cases (corrupted files, permissions, etc.)
- Progress logging and error handling implemented
- XAPK bundles are handled (XAPK is a ZIP containing split APKs)

**Technical Approach**:
- Use `unzip` to extract APK/XAPK files (both are ZIP archives)
- Implement recursive scanning for nested APK files
- Create organized directory structure: `foo.apk` -> `foo.apk.extracted/`
- Log extraction process and maintain manifest of extracted files

### Milestone 2: File Analysis and Business Logic Discovery
**Goal**: Analyze extracted files to identify core business logic components.

**Deliverables**:
- `milestone2_analysis.md` - Comprehensive analysis report

**Acceptance Criteria**:
- All file types catalogued with counts and sizes
- DEX files identified and prioritized
- .NET assemblies (if any) located
- Native libraries (ELF/SO files) inventoried
- Resource files and manifests analyzed
- Core business logic files identified and ranked by importance

**Technical Approach**:
- Use `file` command for magic number detection
- Analyze AndroidManifest.xml for app structure
- Identify main DEX files and additional DEX files
- Scan for native libraries and their architectures
- Document file relationships and dependencies

### Milestone 3: Decompilation of Core Logic
**Goal**: Decompile identified core business logic files using multiple tools.

**Deliverables**:
- `milestone3_decompile.md` - Decompilation findings and results
- `milestone3_classes.md` - Decompiled classes directory structure analysis
- Decompiled source code in organized directories

**Acceptance Criteria**:
- DEX files decompiled using multiple tools (jadx, dex2jar+jd-gui, etc.)
- .NET assemblies decompiled (if present)
- Native ELF binaries analyzed with binwalk and disassemblers
- Each tool's output stored in separate directories: `foo.dex.decompiled.jadx/`
- Obfuscation patterns identified and documented
- Success/failure rates documented per tool and file type

**Technical Approach**:
- **DEX Files**: jadx, dex2jar+jd-gui, baksmali
- **.NET Assemblies**: ilspy, dotPeek-like tools
- **Native Binaries**: binwalk, objdump, radare2
- **Obfuscated Code**: Multiple decompilers for cross-reference
- Automated processing with fallback strategies

### Milestone 4: Architecture Documentation
**Goal**: Document the complete application architecture and business logic.

**Deliverables**:
- `architecture.md` - Comprehensive architecture documentation

**Acceptance Criteria**:
- Complete file and class hierarchy documented
- Technology stack identified (frameworks, libraries)
- Data flow diagrams created
- Code structure and organization explained
- Security implementations analyzed (obfuscation, encryption, certificates)
- REST API endpoints and backend communication documented
- Database schemas and data models identified
- Authentication and authorization mechanisms described
- Aula-specific domain logic documented (messaging, calendar, notifications, school data)

**Technical Approach**:
- Static analysis of decompiled code
- Network configuration analysis
- Security certificate examination
- API endpoint discovery through code analysis
- Data model extraction from ORM configurations
- Documentation with diagrams and code examples

## Non-Functional Requirements

### Environment Setup
- All tools managed via `shell.nix` (no nix flakes)
- No `pip install` - all Python tools via Nix
- Package comments explaining purpose in `shell.nix`
- Reproducible build environment

### Documentation Standards
- Clear, technical documentation with examples
- Code snippets and file paths referenced
- Progress tracking and milestone validation
- Error cases and limitations documented

### Tool Selection Criteria
- Open source and actively maintained
- Available in Nix packages
- Proven effectiveness for Android analysis
- Multiple tools per file type for redundancy

## Risk Mitigation

### Technical Risks
- **Obfuscated Code**: Use multiple decompilation tools
- **Corrupted/Protected APKs**: Implement graceful error handling
- **Large File Sizes**: Stream processing and disk space monitoring
- **Tool Failures**: Fallback strategies and alternative tools
- **XAPK Format**: Handle split APK bundles (extract XAPK first, then process contained APKs)

### Legal/Ethical Considerations
- Analysis for educational/security research purposes
- Respect intellectual property rights
- No redistribution of proprietary code
- Document analysis methodology for transparency

## Success Metrics
- Percentage of files successfully extracted and analyzed
- Number of business logic components identified
- Decompilation success rate per tool and file type
- Completeness of architecture documentation
- Reproducibility of analysis process

## Timeline Estimate
- **Milestone 1**: 1-2 days (extraction automation)
- **Milestone 2**: 2-3 days (analysis and cataloguing)
- **Milestone 3**: 3-5 days (decompilation with multiple tools)
- **Milestone 4**: 2-3 days (architecture documentation)

**Total Project Duration**: 8-13 days
