# Aula Reversed

Unofficial tools for accessing Denmark's Aula school communication platform. The project delivers three things:

1. **`aula-api`** -- Rust library for the Aula REST API (authentication, models, service calls)
2. **`aula-cli`** -- Command-line interface for reading messages, calendar, posts, gallery, documents, presence, and more
3. **`aula-fuse`** -- FUSE filesystem that mounts Aula data as a browsable directory tree (photos viewable with any image viewer)

## CLI commands

Read access to most Aula domains. All commands support `--json` for machine-readable output.

| Command | Description |
|---------|-------------|
| `aula auth login` | Browser-based OIDC login (UniLogin or MitID) |
| `aula auth status` | Show current login state and token expiry |
| `aula auth refresh` | Refresh an expired access token |
| `aula auth logout` | Clear session and tokens |
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

## FUSE filesystem

Mount Aula data as a read-only directory tree. Gallery photos are lazily downloaded on access -- open them directly with `feh`, `eog`, or any image viewer.

```
/tmp/aula/
  posts/           institution feed posts
  messages/        message threads and individual messages
  calendar/        events with details
  gallery/         photo albums with lazy-downloaded media
  documents/       secure documents
  notifications/   notification items
  presence/        children's presence status
```

Pagination appears as nested `page-N/` subdirectories.

```bash
just mount              # mount to /tmp/aula
just umount             # unmount
feh /tmp/aula/gallery/*/  # browse album photos
```

## Getting started

Prerequisites: [Nix package manager](https://nixos.org/download.html) and a browser for login.

```bash
nix-shell               # enter dev environment
just build              # build everything
just run auth login --manual  # authenticate via browser
just run messages list        # list message threads
just mount                    # mount as filesystem
```

## Configuration

`~/.config/aula/config.toml`:

```toml
default_environment = "production"
default_format = "text"
default_profile = "MyChild"
verbose = false
```

All settings can be overridden with CLI flags.

## Authentication

OIDC Authorization Code + PKCE flow through `login.aula.dk`.

- **Manual** (`--manual`, recommended): opens browser, you paste the redirect URL back into the terminal.
- **Automatic** (default): opens browser and captures the auth code via a local callback server. May not work in all environments.

Tokens stored at `~/.local/share/aula/tokens.json` with `0600` permissions.

## Project structure

```
aula/                    Rust workspace
  aula-api/              API client library (auth, models, services)
  aula-cli/              CLI binary (clap, one module per domain)
  aula-fuse/             FUSE filesystem binary
re/                      Reverse engineering analysis documents
secrets/                 Gitignored: dev artifacts, HAR dumps, test fixtures
scripts/                 Helper scripts (login, chrome devtools)
shell.nix                Nix environment
Justfile                 Common recipes (build, test, lint, run, mount)
```

## License

MIT License. See [LICENSE](LICENSE).
