We shall write a prd.apk_decompile.md project requirements file for Android App APK decompilation.
Android apk files are zip files, with other apk files within.
Those apk files contain DEX files or dotNET assemblies, alongside resources and meta-data.

Target app: Aula (com.netcompany.aulanativeprivate) - Denmark's school communication platform by Netcompany A/S.

Milestone 1 - extract:
We shall extract those apk files recursively: Upon every extraction, look for more apks to extract - repeat until all apk files have been extracted.
When extracting e.g. foo.apk, extract to foo.apk.extracted/ directory -- this preserves the origin and hiearchy. Write this as a BASH script, apk_extract.sh and run it.
Handle XAPK format (split APK bundles) which are ZIP files containing multiple APKs.

Milestone 2 - analyze:
When everything has been extracted, analyze files. Look for core buisness logic.
Document findings in milestone2_analysis.md file.

Milestone 3 - decompile:
With core logic files identified, we shall decompile each of these files. Start with the most promising ones. Use file magic, websearch, and automatic decompilation tools until succesful. Apply binwalk to unknown binaries.
First focus on DEX files, then .NET assemblies, then native ELFs.
If core buisness logic is obfuscated, we went to run several decompilation tools - each storing output under their own directory. E.g. foo.dex is decompiled to foo.dex.decompiled.jadx/ directory.
Document findings in milestone3_decompile.md file.

Milestone 4 - document core buisness logic architecture:
Write architecture.md which documents files, classes, technologies, data-flow, code-structure, security, (REST) API calls to backend servers.

--

For all of the above, we shall use nix-shell --run 'tool args' with a shell.nix file. No nix flakes. No pip install. All tool usage shall go into shell.nix file. Each package in shell.nix file has have a comment explaining what it is for.

To determine nix package name for a tool "foo", you can browse @https://search.nixos.org/packages?channel=25.05&type=packages&query=foo
