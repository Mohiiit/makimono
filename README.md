# Makimono (Madara DB Visualizer)

> A Madara RocksDB visualizer shipped as a single command.

[![Deploy to GitHub Pages](https://github.com/Mohiiit/makimono/actions/workflows/deploy.yml/badge.svg)](https://github.com/Mohiiit/makimono/actions/workflows/deploy.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Makimono detects your Madara DB schema version from `.db-version`, installs the matching visualizer toolchain, and runs the **UI + API from a single port**.

## Why “Makimono”?

In Naruto, a *makimono* is a scroll: compact, portable, and used to carry the important stuff.

That’s the vibe here: you point Makimono at a Madara RocksDB directory, and it brings the right visualizer along for the ride (matching your DB schema version).

## Naming (So It's Not Confusing)

- **Makimono**: the end-user CLI you install and run (`makimono run ...`).
- **`makimono-viz`**: the versioned toolchain binary that Makimono downloads and runs for your DB version.
- **Madara DB Visualizer**: the web UI title (served by `makimono-viz`).

This repository was previously named `madara-db-visualizer`.

![Makimono UI](docs/images/09-makimono-viz.png)

## Quickstart (No Docker)

### What You Get

- Single command: `makimono run <path>`
- Zero Docker requirement for end users
- Compatibility clarity via `.db-version`-based toolchains
- One port serving everything: UI `/` + API `/api/*`

### Install

macOS/Linux:
```bash
curl -fsSL https://raw.githubusercontent.com/Mohiiit/makimono/main/install.sh | bash
```

Windows PowerShell:
```powershell
iwr -useb https://raw.githubusercontent.com/Mohiiit/makimono/main/install.ps1 | iex
```

If the install script returns 404s, check that GitHub Releases are reachable from your network and that the repo/tag is correct.

Developer fallback (build from source):
```bash
cargo install --git https://github.com/Mohiiit/makimono.git --bin makimono
```

### Run

Pass either the Madara base path or the RocksDB directory:
```bash
makimono run ~/.madara
makimono run ~/.madara/db

# Or run the bundled sample DB
makimono run ./sample-db
```

Open `http://127.0.0.1:8080`.

## Compatibility (Madara DB Versions)

Madara stores a DB schema version in a `.db-version` file under the base path (next to `db/`).

Makimono reads `.db-version` and selects a matching toolchain release tag:
- Immutable: `N.x.y` (example: `9.0.1`)
- Moving alias: `N` (example: `9`) points to the latest compatible build

If `.db-version` is missing, Makimono falls back to the highest installed toolchain (or errors if none are installed).

Currently validated: `8` and `9`.

Useful overrides:
- `makimono run <path> --db-version <N>`
- `makimono run <path> --offline`

## How It Works

- `makimono` (bootstrapper) resolves the DB directory, reads `.db-version`, downloads the matching toolchain from GitHub Releases, and runs it.
- `makimono-viz` (toolchain) is a single binary that serves the embedded frontend at `/` and the API at `/api/*`.

## Docker Compose (Optional)

If you prefer Docker for local runs:
```bash
./scripts/up.sh                # uses ./sample-db
./scripts/up.sh ~/.madara      # or ~/.madara/db
```

Open `http://localhost:8080`.

## Documentation

- API endpoints: `docs/API.md`
- Development notes: `docs/DEVELOPMENT.md`
- Maintainer scratchpad: `docs/makimono-scratchpad.md`
- Screenshot gallery: `docs/images/`

## License

MIT. See `LICENSE`.
