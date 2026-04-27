<!--
SPDX-FileCopyrightText: 2026 psa-ng project contributors

SPDX-FileType: DOCUMENTATION
SPDX-License-Identifier: GPL-3.0-only
-->

# psa-ng Specification

---

The key words "*MUST*", "*MUST NOT*", "*REQUIRED*", "*SHALL*", "*SHALL NOT*", "*SHOULD*", "*SHOULD NOT*", "*RECOMMENDED*", "*MAY*", and "*OPTIONAL*" in this document are to be interpreted as described in [IETF BCP14 (RFC2119 & RFC8174)](https://www.rfc-editor.org/info/bcp14)

---

psa-ng is a Rust reimplementation of the [psa_car_controller](https://github.com/flobz/psa_car_controller) project. It provides remote control and monitoring of PSA group vehicles (Peugeot, Citroën, Opel/Vauxhall, DS) via the PSA Connected Car v4 API, with a local web dashboard for visualization and control.

The project is split into two main modules:
- **psa-api**: PSA API client library handling authentication, vehicle queries, and remote commands
- **psa-web**: Web server providing REST endpoints and an interactive dashboard

## Architecture requirements

### Modular architecture

`req~modular-architecture~1`:
The project *MUST* be organized as a Cargo workspace with separate crates for the PSA API client library and the web server application.

Needs: impl

### Rust best practices

`req~rust-best-practices~1`:
All crates *MUST* compile without warnings under `#[deny(warnings)]` and *MUST* pass `clippy` with default lints.

Needs: impl, utest

### Stable dependencies

`req~stable-dependencies~1`:
The project *MUST* only depend on stable, well-maintained crates that are widely adopted in the Rust ecosystem.

Needs: impl

## PSA API authentication requirements

### OAuth2 authentication

`req~oauth2-authentication~1`:
The PSA API client *MUST* implement the OAuth2 authorization flow to obtain and manage access tokens for the PSA Connected Car v4 API.

Needs: impl, utest

### Token refresh

`req~token-refresh~1`:
The PSA API client *MUST* automatically refresh expired OAuth2 access tokens using the stored refresh token before making API requests.

Needs: impl, utest

Depends:
- req~oauth2-authentication~1

### Credential persistence

`req~credential-persistence~1`:
The PSA API client *MUST* persist OAuth2 tokens and credentials to a local file so that re-authentication is not required on restart.

Needs: impl, utest

Depends:
- req~oauth2-authentication~1

## PSA API client requirements

### Vehicle list retrieval

`req~vehicle-list~1`:
The PSA API client *MUST* retrieve the list of vehicles associated with the authenticated user account.

Needs: impl, utest

Depends:
- req~oauth2-authentication~1

### Vehicle status retrieval

`req~vehicle-status~1`:
The PSA API client *MUST* retrieve the current status of a vehicle, including battery level, charging state, odometer reading, and last-known position.

Needs: impl, utest

Depends:
- req~vehicle-list~1

### Vehicle wakeup

`req~vehicle-wakeup~1`:
The PSA API client *MUST* support sending a wakeup request to force a vehicle to report its current status.

Needs: impl, utest

### Charge control

`req~charge-control~1`:
The PSA API client *MUST* support starting and stopping vehicle charging via remote commands.

Needs: impl, utest

### Charge threshold

`req~charge-threshold~1`:
The PSA API client *MUST* support setting a battery charge threshold percentage to limit charging.

Needs: impl, utest

### Charge scheduling

`req~charge-scheduling~1`:
The PSA API client *MUST* support setting a scheduled stop hour for charging to enable off-peak charging.

Needs: impl, utest

### Preconditioning control

`req~preconditioning-control~1`:
The PSA API client *MUST* support starting and stopping air conditioning preconditioning.

Needs: impl, utest

### Door lock control

`req~door-lock-control~1`:
The PSA API client *MUST* support locking and unlocking vehicle doors via remote commands.

Needs: impl, utest

### Lights and horn control

`req~lights-horn-control~1`:
The PSA API client *MUST* support flashing lights and honking the horn via remote commands.

Needs: impl, utest

## Data persistence requirements

### Status history storage

`req~status-history~1`:
The application *MUST* persist vehicle status snapshots over time to enable historical analysis and dashboard visualization.

Needs: impl, utest

### Trip recording

`req~trip-recording~1`:
The application *MUST* record and persist trip data including start/end positions, distance, and energy consumption.

Needs: impl, utest

### Charging session recording

`req~charging-session-recording~1`:
The application *MUST* record and persist charging session data including start time, end time, energy charged, and battery level changes.

Needs: impl, utest

## Web server requirements

### HTTP server

`req~http-server~1`:
The web module *MUST* provide an HTTP server using a lightweight, well-maintained Rust web framework.

Needs: impl

### Vehicle status endpoint

`req~vehicle-status-endpoint~1`:
The web server *MUST* expose an endpoint that returns the current vehicle status as JSON.

Needs: impl, utest

Depends:
- req~vehicle-status~1

### Charge control endpoint

`req~charge-control-endpoint~1`:
The web server *MUST* expose endpoints for starting/stopping charging, setting charge threshold, and setting charge schedule.

Needs: impl, utest

Depends:
- req~charge-control~1
- req~charge-threshold~1
- req~charge-scheduling~1

### Preconditioning endpoint

`req~preconditioning-endpoint~1`:
The web server *MUST* expose an endpoint for starting and stopping air conditioning preconditioning.

Needs: impl, utest

Depends:
- req~preconditioning-control~1

### Door lock endpoint

`req~door-lock-endpoint~1`:
The web server *MUST* expose an endpoint for locking and unlocking vehicle doors.

Needs: impl, utest

Depends:
- req~door-lock-control~1

### Lights and horn endpoint

`req~lights-horn-endpoint~1`:
The web server *MUST* expose an endpoint for flashing lights and honking the horn.

Needs: impl, utest

Depends:
- req~lights-horn-control~1

### Vehicle wakeup endpoint

`req~wakeup-endpoint~1`:
The web server *MUST* expose an endpoint to trigger a vehicle wakeup.

Needs: impl, utest

Depends:
- req~vehicle-wakeup~1

### Settings endpoint

`req~settings-endpoint~1`:
The web server *MUST* expose endpoints for reading and updating application configuration.

Needs: impl, utest

### Trips endpoint

`req~trips-endpoint~1`:
The web server *MUST* expose an endpoint that returns recorded trip data.

Needs: impl, utest

Depends:
- req~trip-recording~1

### Charging sessions endpoint

`req~charging-sessions-endpoint~1`:
The web server *MUST* expose an endpoint that returns recorded charging session data.

Needs: impl, utest

Depends:
- req~charging-session-recording~1

## Dashboard requirements

### Dashboard overview page

`req~dashboard-overview~1`:
The web server *MUST* serve a dashboard page that displays a summary of vehicle status including battery level, charging state, and last-known position.

Needs: impl

Depends:
- req~vehicle-status~1

### Charge management page

`req~charge-management-page~1`:
The web server *MUST* serve a page with forms to control charging: start/stop charge, set threshold percentage, and set charge schedule.

Needs: impl

Depends:
- req~charge-control-endpoint~1

### Trip display page

`req~trip-display-page~1`:
The web server *MUST* serve a page that displays recorded trips in a tabular format.

Needs: impl

Depends:
- req~trips-endpoint~1

### Settings page

`req~settings-page~1`:
The web server *MUST* serve a page with forms for managing application configuration including electricity pricing.

Needs: impl

Depends:
- req~settings-endpoint~1

### Clean web styling

`req~clean-web-styling~1`:
The web dashboard *MUST* use a clean, simple, and responsive CSS styling that works well on both desktop and mobile browsers.

Needs: impl

## Configuration requirements

### Configuration file

`req~configuration-file~1`:
The application *MUST* load configuration from a TOML file, including PSA API credentials, electricity pricing, and server settings.

Needs: impl, utest

### Electricity pricing

`req~electricity-pricing~1`:
The configuration *MUST* support setting an electricity price per kWh, with *OPTIONAL* support for separate day and night pricing with configurable time ranges.

Needs: impl, utest

Depends:
- req~configuration-file~1

## Security requirements

### API bearer token authentication

`req~api-bearer-auth~1`:
The web server *MUST* require a configurable bearer token on all `/api/*` endpoints, rejecting unauthenticated requests with HTTP 401.

Needs: impl, utest

### HTML output escaping

`req~html-output-escaping~1`:
The web server *MUST* escape all dynamic values inserted into HTML templates to prevent cross-site scripting (XSS) attacks.

Needs: impl, utest

### Request body size limit

`req~request-body-limit~1`:
The web server *MUST* enforce a maximum request body size to prevent denial-of-service via oversized payloads.

Needs: impl

### Security response headers

`req~security-headers~1`:
The web server *MUST* set security-related HTTP response headers including `Content-Security-Policy`, `X-Content-Type-Options`, `X-Frame-Options`, and `Referrer-Policy`.

Needs: impl, utest

### Sanitized error responses

`req~sanitized-errors~1`:
API error responses *MUST NOT* expose internal implementation details such as file paths, token states, or upstream API URLs to clients.

Needs: impl, utest

### Dependency vulnerability scanning

`req~dependency-audit~1`:
The CI pipeline *MUST* include automated dependency vulnerability scanning via `cargo audit`.

Needs: impl

## Future requirements

The following requirements are specified but not yet implemented. They document planned functionality.

### Status polling

`req~status-polling~1`:
The application *SHOULD* periodically poll vehicle status from the PSA API and persist snapshots, trips, and charging sessions to the database automatically.

Needs: impl, utest

Depends:
- req~vehicle-status~1
- req~status-history~1
- req~trip-recording~1
- req~charging-session-recording~1
