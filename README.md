<!-- [impl->req~modular-architecture~1] -->

[![TSF Score](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fanotherdaniel.github.io%2Fpsa-ng%2Ftrustable%2Fscore.json&query=%24.scores%5B0%5D.score&label=TSF%20Score&color=blue)](https://anotherdaniel.github.io/psa-ng/trustable/dashboard.html)
[![TSF Report](https://img.shields.io/github/v/release/AnotherDaniel/psa-ng?label=TSF%20Report&color=green)](https://anotherdaniel.github.io/psa-ng/trustable/trustable_report_for_psa-ng.html)

# psa-ng

A Rust reimplementation of [psa_car_controller](https://github.com/flobz/psa_car_controller) — remote control and monitoring for PSA group vehicles (Peugeot, Citroën, Opel/Vauxhall, DS) via the PSA Connected Car v4 API.

## Features

- **Vehicle status** — battery level, charging state, mileage, position, autonomy
- **Charge control** — start/stop charging, set battery threshold, schedule off-peak charging
- **Climate control** — start/stop air conditioning preconditioning
- **Door locks** — lock and unlock remotely
- **Lights & horn** — flash lights, honk horn
- **Dashboard** — clean, responsive web UI for status overview and control
- **Data recording** — trip history, charging sessions, status snapshots (SQLite)
- **REST API** — full JSON API for all operations, suitable for automation and integration

## Architecture

The project is a Cargo workspace with two crates:

| Crate | Purpose |
|-------|---------|
| **psa-api** | PSA Connected Car v4 API client library — OAuth2 auth, token management, vehicle queries, remote commands |
| **psa-web** | Web server (axum) — REST endpoints, HTML dashboard, SQLite persistence |

### Key dependencies

| Crate | Role |
|-------|------|
| [axum](https://crates.io/crates/axum) | HTTP framework |
| [reqwest](https://crates.io/crates/reqwest) | HTTP client for PSA API |
| [rusqlite](https://crates.io/crates/rusqlite) | SQLite storage (bundled) |
| [tokio](https://crates.io/crates/tokio) | Async runtime |
| [serde](https://crates.io/crates/serde) / [toml](https://crates.io/crates/toml) | Configuration & serialization |

## Getting Started

### Prerequisites

- **Rust 1.85+** (edition 2024)
- A **PSA Connected Car API** account with `client_id` and `client_secret` (register at [developer.groupe-psa.io](https://developer.groupe-psa.io/))
- A PSA brand account (the one used with the official mobile app)

### Build

```bash
git clone https://github.com/AnotherDaniel/psa-ng.git
cd psa-ng
cargo build --release
```

The compiled binary is at `target/release/psa-web`.

### Configure

Copy the example configuration and edit it with your credentials:

```bash
cp config.toml.example config.toml
```

Edit `config.toml`:

```toml
[psa]
client_id = "your_client_id"
client_secret = "your_client_secret"
brand = "peugeot"   # peugeot, citroen, ds, opel, vauxhall

[server]
host = "127.0.0.1"
port = 5000

[electricity]
price_per_kwh = 0.15
currency = "EUR"
```

See the [Configuration Reference](#configuration-reference) below for all options.

### Run

```bash
# Development
cargo run -p psa-web -- config.toml

# Production (release build)
./target/release/psa-web config.toml
```

Open [http://127.0.0.1:5000](http://127.0.0.1:5000) in your browser.

### Docker

Build and run with Docker Compose:

```bash
# Create config (set host to 0.0.0.0 for Docker)
cp config.toml.example config.toml
# Edit config.toml: set host = "0.0.0.0" under [server]

# Build and start
docker compose up -d

# View logs
docker compose logs -f

# Stop
docker compose down
```

Or build and run the image directly:

```bash
docker build -t psa-ng .
docker run -d --name psa-ng \
  -p 5000:5000 \
  -v ./config.toml:/app/config.toml:ro \
  -v psa-data:/app/data \
  psa-ng
```

> **Note:** Set `host = "0.0.0.0"` in `config.toml` when running in Docker so the server is reachable from outside the container. The SQLite database and token file are persisted in the `psa-data` volume.

## Configuration Reference

The application reads a single TOML file (default: `config.toml`, or pass a path as the first CLI argument).

### `[psa]` — PSA API credentials

| Key | Required | Default | Description |
|-----|----------|---------|-------------|
| `client_id` | **yes** | — | OAuth2 client ID from the PSA developer portal |
| `client_secret` | **yes** | — | OAuth2 client secret |
| `brand` | **yes** | — | Vehicle brand: `peugeot`, `citroen`, `ds`, `opel`, `vauxhall` |
| `api_base_url` | no | `https://api.groupe-psa.com/connectedcar/v4` | PSA API base URL (override for testing) |
| `token_file` | no | `data/token.json` | Path to persist OAuth2 tokens |

### `[server]` — Web server

| Key | Required | Default | Description |
|-----|----------|---------|-------------|
| `host` | no | `127.0.0.1` | Listen address. Use `0.0.0.0` for all interfaces |
| `port` | no | `5000` | Listen port |
| `data_dir` | no | `data` | Directory for database and token files |

### `[electricity]` — Pricing

| Key | Required | Default | Description |
|-----|----------|---------|-------------|
| `price_per_kwh` | no | `0.0` | Electricity price per kWh |
| `currency` | no | `EUR` | Currency symbol/code for display |
| `night_price_per_kwh` | no | — | Reduced night-rate price per kWh |
| `night_start_hour` | no | — | Night rate start hour (0–23) |
| `night_start_minute` | no | — | Night rate start minute (0–59) |
| `night_end_hour` | no | — | Night rate end hour (0–23) |
| `night_end_minute` | no | — | Night rate end minute (0–59) |

#### Night pricing example

```toml
[electricity]
price_per_kwh = 0.25
night_price_per_kwh = 0.12
night_start_hour = 22
night_start_minute = 0
night_end_hour = 6
night_end_minute = 0
currency = "EUR"
```

## REST API

All endpoints return JSON. The base URL defaults to `http://127.0.0.1:5000`.

### Vehicle status

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/vehicles` | List all vehicles |
| `GET` | `/api/vehicles/{id}/status` | Get vehicle status |
| `POST` | `/api/vehicles/{id}/wakeup` | Force status refresh |

### Charge control

| Method | Path | Body | Description |
|--------|------|------|-------------|
| `POST` | `/api/vehicles/{id}/charge` | `{"start": true}` | Start or stop charging |
| `POST` | `/api/vehicles/{id}/charge/threshold` | `{"percentage": 80}` | Set charge limit (%) |
| `POST` | `/api/vehicles/{id}/charge/schedule` | `{"hour": 6, "minute": 0}` | Set charge stop time |

### Vehicle control

| Method | Path | Body | Description |
|--------|------|------|-------------|
| `POST` | `/api/vehicles/{id}/preconditioning` | `{"start": true}` | Start/stop AC preconditioning |
| `POST` | `/api/vehicles/{id}/doors` | `{"lock": true}` | Lock/unlock doors |
| `POST` | `/api/vehicles/{id}/lights` | `{"duration": 10}` | Flash lights (seconds) |
| `POST` | `/api/vehicles/{id}/horn` | `{"count": 3}` | Honk horn (number of times) |

### Data & settings

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/trips` | Get recorded trips (optional `?vin=...`) |
| `GET` | `/api/charging-sessions` | Get charging sessions (optional `?vin=...`) |
| `GET` | `/api/settings` | Get electricity pricing config |
| `POST` | `/api/settings` | Update electricity pricing config |

### Example: curl

```bash
# Get vehicle list
curl http://localhost:5000/api/vehicles

# Start charging
curl -X POST http://localhost:5000/api/vehicles/VEHICLE_ID/charge \
  -H "Content-Type: application/json" \
  -d '{"start": true}'

# Set charge threshold to 80%
curl -X POST http://localhost:5000/api/vehicles/VEHICLE_ID/charge/threshold \
  -H "Content-Type: application/json" \
  -d '{"percentage": 80}'

# Lock doors
curl -X POST http://localhost:5000/api/vehicles/VEHICLE_ID/doors \
  -H "Content-Type: application/json" \
  -d '{"lock": true}'
```

## Dashboard

The web dashboard is available at the root URL and provides four pages:

| Page | URL | Content |
|------|-----|---------|
| **Dashboard** | `/` | Vehicle status overview — battery, charging state, mileage, autonomy, position |
| **Charge** | `/charge` | Charge control forms — start/stop, threshold, schedule |
| **Trips** | `/trips` | Recorded trip history table with distance and efficiency |
| **Settings** | `/settings` | Electricity pricing configuration |

The dashboard uses a clean, responsive design that works on both desktop and mobile browsers. No JavaScript frameworks are required — it is pure HTML/CSS with minimal vanilla JS for form submissions.

## Data Storage

Vehicle status, trips, and charging sessions are stored in a SQLite database at `{data_dir}/psa-ng.db` (default: `data/psa-ng.db`). The database is created automatically on first run.

| Table | Content |
|-------|---------|
| `status_history` | Timestamped vehicle status snapshots |
| `trips` | Trip records with start/end positions, distance, energy |
| `charging_sessions` | Charging session records with levels, energy, cost |

## Development

```bash
# Run tests
cargo test --workspace

# Run clippy lints
cargo clippy --workspace

# Run in development mode with logging
RUST_LOG=info cargo run -p psa-web -- config.toml
```

### Running CI workflows locally with `act`

You can run the GitHub Actions workflows locally using [`act`](https://nektosact.com/). The repository includes an `.actrc` with sensible defaults for macOS (Apple Silicon).

**Prerequisites:** [Docker](https://www.docker.com/) and `act` (`brew install act`).

```bash
# Run the PR-level checks (fmt, clippy, test, doc, deny)
act -W .github/workflows/check.yaml

# Run a single job
act -W .github/workflows/check.yaml -j test

# Run nightly checks (MSRV, coverage, deny)
act -W .github/workflows/nightly.yaml

# Dry-run any workflow (no Docker containers started)
act -W .github/workflows/check.yaml -n

# List all available jobs
act -l
```

The **release** workflow needs a GitHub token for tsffer/tsflink steps. Copy the secrets template and fill in your token:

```bash
cp .github/.act-secrets.example .github/.act-secrets
# Edit .github/.act-secrets with your GitHub PAT

act -W .github/workflows/release.yaml \
  -e .github/act-event-tag.json \
  --secret-file .github/.act-secrets
```

> **Note:** The release workflow's artifact upload, OFT, and Pages deployment steps depend on GitHub infrastructure and may not fully succeed locally. The check and nightly workflows run locally without issues.

### Project structure

```
psa-ng/
├── Cargo.toml              # Workspace root
├── config.toml.example     # Example configuration
├── psa-api/
│   └── src/
│       ├── lib.rs           # Crate root
│       ├── auth.rs          # OAuth2 client (token lifecycle)
│       ├── client.rs        # PSA API client (all vehicle operations)
│       ├── config.rs        # Configuration types
│       ├── error.rs         # Error types
│       └── models.rs        # API data models
├── psa-web/
│   └── src/
│       ├── main.rs          # Entry point
│       ├── routes.rs        # REST + page handlers
│       ├── templates.rs     # HTML/CSS rendering
│       ├── db.rs            # SQLite persistence
│       └── state.rs         # Shared application state
├── docs/
│   └── specification.md     # OFT requirements specification
└── trustable/
    └── psa-ng/              # TSF quality statements
```

## Quality & Traceability

This project uses the [Trustable Software Framework](https://trustable.io/) (TSF) with [OpenFastTrace](https://github.com/itsallcode/openfasttrace) (OFT) for requirements traceability:

- **Specification** — formal `req~` requirements in [docs/specification.md](docs/specification.md)
- **OFT markers** — `[impl->req~...]` and `[utest->req~...]` tags in source code link implementation and tests back to requirements
- **TSF statements** — quality claims in [trustable/psa-ng/](trustable/psa-ng/) with evidence plans
- **CI evidence** — automated collection via `tsffer` in the release workflow

## License

[GPL-3.0-only](LICENSE)

## Acknowledgements

This project is a Rust reimplementation inspired by [flobz/psa_car_controller](https://github.com/flobz/psa_car_controller). It uses the PSA Connected Car v4 API.
