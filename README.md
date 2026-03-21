# Aula Reversed

Reverse engineering project for the Aula Android app (`com.netcompany.aulanativeprivate` v2.15.4) -- Denmark's school communication platform by Netcompany A/S. The project produces two things:

1. **Reverse engineering analysis** of the APK (decompilation, architecture docs, API endpoint mapping)
2. **A Rust CLI tool** (`aula-cli`) for interacting with the Aula platform from the command line

## What the CLI can do

The CLI provides read access to most Aula domains. All commands support `--json` for machine-readable output.

| Command | Description |
|---------|-------------|
| `aula auth login` | Browser-based OIDC login (UniLogin level 2 or MitID level 3) |
| `aula auth logout` | Clear session and tokens |
| `aula auth status` | Show current login state and token expiry |
| `aula auth refresh` | Refresh an expired access token |
| `aula messages list` | List message threads |
| `aula messages read <id>` | Read a specific thread |
| `aula messages folders` | List message folders |
| `aula calendar today` | Today's calendar events |
| `aula calendar week` | This week's events |
| `aula calendar show <id>` | Show a specific event |
| `aula presence status` | Children's current presence |
| `aula presence schedule` | This week's presence schedule |
| `aula posts list` | List institution feed posts |
| `aula posts show <id>` | Show a specific post |
| `aula gallery list` | List photo albums |
| `aula gallery show <id>` | Show media in an album |
| `aula documents list` | List secure documents |
| `aula documents show <id>` | Show a specific document |
| `aula notifications list` | List recent notifications |
| `aula search <query>` | Search profiles and groups |
| `aula groups list` | List groups for an institution profile |
| `aula groups show <id>` | Show group details |
| `aula groups members <id>` | Show group members |
| `aula profile me` | Show your profile |
| `aula profile master-data` | Show name, email, phone |
| `aula config ...` | View and manage CLI configuration |

Global flags: `--json`, `--env <environment>`, `--verbose`, `--profile <name>`.

Supported environments: `production`, `preprod`, `hotfix`, `test1`, `test3`, `dev1`, `dev3`, `dev11`.

## Prerequisites

- [Nix package manager](https://nixos.org/download.html) (provides all build tools via `shell.nix`)
- A browser for the OIDC login flow

## Running the CLI

Enter the Nix shell, then use `just` or `cargo` directly:

```bash
# Enter the development environment
nix-shell

# Build
just build

# Run the CLI
just run --help
just run auth login
just run messages list --limit 10

# Or use cargo directly
cargo run --manifest-path aula/Cargo.toml --bin aula-cli -- auth login

# Run tests, linting, formatting check
just e2e

# Run live E2E tests against the real API (requires auth token)
just e2e-live
```

The Justfile also has convenience recipes for common queries. Run `just` to list them.

## Configuration

The CLI reads settings from `~/.config/aula/config.toml`:

```toml
default_environment = "production"
default_format = "text"
default_profile = "MyChild"
verbose = false
```

All settings can be overridden with CLI flags.

## Authentication and token storage

The CLI authenticates via the same OIDC Authorization Code + PKCE flow the mobile app uses, through `login.aula.dk`.

**Login modes:**

- **Automatic** (default): starts a local HTTP callback server, opens your browser, and captures the auth code automatically.
- **Manual** (`--manual`): for environments where localhost redirect is rejected. You paste the redirect URL from your browser.

**Token storage:**

After login, tokens are persisted to `~/.local/share/aula/tokens.json` (or the platform-appropriate XDG data directory). The file is created with `0600` permissions (owner read/write only).

The `secrets/` directory in this repo is gitignored and used for development artifacts like HAR captures and test fixtures -- it is not part of the CLI's token storage.

**Token lifecycle:**

```
aula auth login          # obtain tokens
aula auth status         # check expiry
aula auth refresh        # refresh without re-login
aula auth logout         # clear tokens
```

## Project structure

```
aula/                    Rust workspace
  aula-api/              API client library (auth, models, services)
  aula-cli/              CLI binary (clap-based, one module per domain)
re/                      Reverse engineering artifacts and analysis
  architecture.md        APK architecture documentation
  prd.apk_decompile.md   Project requirements (4 milestones)
  milestone2_analysis.md File analysis findings
  auth_flow.md           OIDC authentication flow analysis
  api_endpoints.md       Discovered API endpoints
  data_models.md         Data model documentation
  ...                    (many more analysis documents)
secrets/                 Gitignored: tokens, HAR dumps, test fixtures
scripts/                 Utility scripts
shell.nix                Nix environment (Rust toolchain, RE tools)
Justfile                 Common recipes (build, test, lint, run, showcase)
```

## Reverse engineering docs

Key analysis documents under `re/`:

| Document | Contents |
|----------|----------|
| `architecture.md` | Full app architecture: Xamarin.Android stack, DI, assembly structure |
| `prd.apk_decompile.md` | Original project requirements with 4 milestones |
| `auth_flow.md` | OIDC login flow, token storage, certificate pinning |
| `api_endpoints.md` | Discovered REST API endpoints and their parameters |
| `data_models.md` | Domain models: messages, calendar, presence, posts, etc. |
| `milestone2_analysis.md` | File-level analysis of extracted APK contents |
| `firebase_analysis.md` | Firebase Cloud Messaging integration |
| `security_analysis.md` | Security mechanisms: pinning, encryption, secure storage |

## License

This is a personal reverse engineering project for educational and interoperability purposes. The Aula platform is owned by Netcompany A/S.
