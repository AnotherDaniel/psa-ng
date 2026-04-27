// [impl->req~clean-web-styling~1]

//! Server-side HTML rendering for the psa-ng web interface.

use psa_api::config::AppConfig;
use psa_api::models::{Trip, VehicleOverview};

// [impl->req~html-output-escaping~1]
/// Escape HTML special characters to prevent XSS attacks.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Wrap page content in the common HTML shell (head, nav, footer).
fn base_html(title: &str, active_nav: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title} — psa-ng</title>
    <style>
{CSS}
    </style>
</head>
<body>
    <nav class="navbar">
        <div class="container nav-container">
            <a href="/" class="nav-brand">psa-ng</a>
            <div class="nav-links">
                <a href="/" class="{dashboard_active}">Dashboard</a>
                <a href="/charge" class="{charge_active}">Charge</a>
                <a href="/trips" class="{trips_active}">Trips</a>
                <a href="/settings" class="{settings_active}">Settings</a>
            </div>
        </div>
    </nav>
    <main class="container">
        {content}
    </main>
    <footer class="footer">
        <div class="container">psa-ng &mdash; PSA Connected Car Controller</div>
    </footer>
</body>
</html>"#,
        title = title,
        CSS = CSS,
        content = content,
        dashboard_active = if active_nav == "dashboard" {
            "active"
        } else {
            ""
        },
        charge_active = if active_nav == "charge" { "active" } else { "" },
        trips_active = if active_nav == "trips" { "active" } else { "" },
        settings_active = if active_nav == "settings" {
            "active"
        } else {
            ""
        },
    )
}

const CSS: &str = r#"
:root {
    --bg: #f7f8fa;
    --surface: #ffffff;
    --primary: #2563eb;
    --primary-hover: #1d4ed8;
    --text: #1e293b;
    --text-muted: #64748b;
    --border: #e2e8f0;
    --success: #16a34a;
    --warning: #d97706;
    --danger: #dc2626;
    --radius: 8px;
}
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: var(--bg);
    color: var(--text);
    line-height: 1.6;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}
.container { max-width: 960px; margin: 0 auto; padding: 0 1rem; }
.navbar {
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    padding: 0.75rem 0;
    position: sticky;
    top: 0;
    z-index: 100;
}
.nav-container { display: flex; align-items: center; justify-content: space-between; }
.nav-brand {
    font-weight: 700;
    font-size: 1.25rem;
    color: var(--primary);
    text-decoration: none;
}
.nav-links { display: flex; gap: 0.25rem; }
.nav-links a {
    padding: 0.5rem 1rem;
    border-radius: var(--radius);
    text-decoration: none;
    color: var(--text-muted);
    font-weight: 500;
    font-size: 0.9rem;
    transition: background 0.15s, color 0.15s;
}
.nav-links a:hover { background: var(--bg); color: var(--text); }
.nav-links a.active { background: var(--primary); color: #fff; }
main { flex: 1; padding: 2rem 0; }
h1 { font-size: 1.75rem; margin-bottom: 1.5rem; }
h2 { font-size: 1.25rem; margin-bottom: 1rem; color: var(--text-muted); }
.card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 1.5rem;
    margin-bottom: 1rem;
}
.card-title { font-weight: 600; font-size: 1.1rem; margin-bottom: 0.75rem; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 1rem; }
.stat { display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--border); }
.stat:last-child { border-bottom: none; }
.stat-label { color: var(--text-muted); font-size: 0.9rem; }
.stat-value { font-weight: 600; }
.badge {
    display: inline-block;
    padding: 0.2rem 0.6rem;
    border-radius: 999px;
    font-size: 0.8rem;
    font-weight: 600;
}
.badge-success { background: #dcfce7; color: var(--success); }
.badge-warning { background: #fef3c7; color: var(--warning); }
.badge-danger { background: #fee2e2; color: var(--danger); }
.badge-neutral { background: var(--bg); color: var(--text-muted); }
.btn {
    display: inline-block;
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: var(--radius);
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    text-decoration: none;
    transition: background 0.15s;
}
.btn-primary { background: var(--primary); color: #fff; }
.btn-primary:hover { background: var(--primary-hover); }
.btn-success { background: var(--success); color: #fff; }
.btn-danger { background: var(--danger); color: #fff; }
.btn-sm { padding: 0.4rem 0.8rem; font-size: 0.85rem; }
.form-group { margin-bottom: 1rem; }
.form-group label { display: block; font-weight: 500; margin-bottom: 0.35rem; font-size: 0.9rem; }
.form-group input, .form-group select {
    width: 100%;
    padding: 0.6rem 0.8rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 0.9rem;
    background: var(--surface);
}
.form-group input:focus, .form-group select:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
}
.form-row { display: flex; gap: 1rem; flex-wrap: wrap; }
.form-row .form-group { flex: 1; min-width: 200px; }
table { width: 100%; border-collapse: collapse; }
th, td {
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid var(--border);
    font-size: 0.9rem;
}
th { font-weight: 600; color: var(--text-muted); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.03em; }
tr:hover { background: var(--bg); }
.empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-muted);
}
.empty-state p { font-size: 1.1rem; margin-bottom: 0.5rem; }
.footer {
    border-top: 1px solid var(--border);
    padding: 1rem 0;
    color: var(--text-muted);
    font-size: 0.85rem;
    text-align: center;
}
.alert { padding: 1rem; border-radius: var(--radius); margin-bottom: 1rem; font-size: 0.9rem; }
.alert-info { background: #eff6ff; border: 1px solid #bfdbfe; color: #1e40af; }
@media (max-width: 600px) {
    .nav-links a { padding: 0.4rem 0.6rem; font-size: 0.8rem; }
    .grid { grid-template-columns: 1fr; }
    .form-row { flex-direction: column; }
}
"#;

/// Render the vehicle dashboard overview page.
pub fn render_dashboard(vehicles: &[VehicleOverview]) -> String {
    let content = if vehicles.is_empty() {
        r#"<h1>Dashboard</h1>
        <div class="empty-state">
            <p>No vehicles found</p>
            <p style="font-size: 0.9rem;">Check your configuration and ensure you have completed the OAuth2 setup.</p>
        </div>"#
            .to_string()
    } else {
        let cards: String = vehicles
            .iter()
            .map(|v| {
                let battery_str = v
                    .battery_level
                    .map(|b| format!("{b:.0}%"))
                    .unwrap_or_else(|| "—".to_string());
                let charge_badge = match v.charging_status.as_deref() {
                    Some("InProgress") => r#"<span class="badge badge-success">Charging</span>"#.to_string(),
                    Some("Disconnected") => {
                        r#"<span class="badge badge-neutral">Disconnected</span>"#.to_string()
                    }
                    Some(s) => format!(r#"<span class="badge badge-warning">{}</span>"#, escape_html(s)),
                    None => r#"<span class="badge badge-neutral">Unknown</span>"#.to_string(),
                };
                let mileage = v
                    .mileage_km
                    .map(|m| format!("{m:.0} km"))
                    .unwrap_or_else(|| "—".to_string());
                let autonomy = v
                    .autonomy_km
                    .map(|a| format!("{a:.0} km"))
                    .unwrap_or_else(|| "—".to_string());
                let label = escape_html(
                    v.label
                        .as_deref()
                        .or(v.brand.as_deref())
                        .unwrap_or("Vehicle"),
                );
                let vin = escape_html(&v.vin);
                let updated = v
                    .last_updated
                    .map(|d| d.format("%Y-%m-%d %H:%M UTC").to_string())
                    .unwrap_or_else(|| "—".to_string());

                format!(
                    r#"<div class="card">
                    <div class="card-title">{label} <span style="font-weight:400; color: var(--text-muted); font-size: 0.85rem;">{vin}</span></div>
                    <div class="stat"><span class="stat-label">Battery</span><span class="stat-value">{battery_str}</span></div>
                    <div class="stat"><span class="stat-label">Charging</span><span class="stat-value">{charge_badge}</span></div>
                    <div class="stat"><span class="stat-label">Autonomy</span><span class="stat-value">{autonomy}</span></div>
                    <div class="stat"><span class="stat-label">Mileage</span><span class="stat-value">{mileage}</span></div>
                    <div class="stat"><span class="stat-label">Last Updated</span><span class="stat-value">{updated}</span></div>
                </div>"#,
                    label = label,
                    vin = vin,
                    battery_str = battery_str,
                    charge_badge = charge_badge,
                    autonomy = autonomy,
                    mileage = mileage,
                    updated = updated,
                )
            })
            .collect();

        format!(
            r#"<h1>Dashboard</h1>
            <div class="grid">{cards}</div>"#
        )
    };

    base_html("Dashboard", "dashboard", &content)
}

/// Render the charge management page.
pub fn render_charge_page(authenticated: bool) -> String {
    let content = if !authenticated {
        r#"<h1>Charge Control</h1>
        <div class="alert alert-info">Authentication required. Please configure your credentials first.</div>"#.to_string()
    } else {
        r#"<h1>Charge Control</h1>
        <div class="card">
            <div class="card-title">Start / Stop Charging</div>
            <p style="color: var(--text-muted); margin-bottom: 1rem; font-size: 0.9rem;">Send a charge command to your vehicle.</p>
            <div style="display: flex; gap: 0.5rem;">
                <button class="btn btn-success btn-sm" onclick="sendCharge(true)">Start Charge</button>
                <button class="btn btn-danger btn-sm" onclick="sendCharge(false)">Stop Charge</button>
            </div>
        </div>
        <div class="card">
            <div class="card-title">Charge Threshold</div>
            <div class="form-group">
                <label for="threshold">Battery limit (%)</label>
                <input type="number" id="threshold" min="20" max="100" step="5" value="80">
            </div>
            <button class="btn btn-primary btn-sm" onclick="setThreshold()">Set Threshold</button>
        </div>
        <div class="card">
            <div class="card-title">Charge Schedule</div>
            <p style="color: var(--text-muted); margin-bottom: 1rem; font-size: 0.9rem;">Set the time to stop charging (for off-peak hours).</p>
            <div class="form-row">
                <div class="form-group">
                    <label for="sched-hour">Hour</label>
                    <input type="number" id="sched-hour" min="0" max="23" value="6">
                </div>
                <div class="form-group">
                    <label for="sched-minute">Minute</label>
                    <input type="number" id="sched-minute" min="0" max="59" value="0">
                </div>
            </div>
            <button class="btn btn-primary btn-sm" onclick="setSchedule()">Set Schedule</button>
        </div>
        <script>
        const VID = 'default';
        async function sendCharge(start) {
            await fetch(`/api/vehicles/${VID}/charge`, {method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify({start})});
            alert(start ? 'Charge started' : 'Charge stopped');
        }
        async function setThreshold() {
            const p = parseInt(document.getElementById('threshold').value);
            await fetch(`/api/vehicles/${VID}/charge/threshold`, {method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify({percentage:p})});
            alert('Threshold set to ' + p + '%');
        }
        async function setSchedule() {
            const h = parseInt(document.getElementById('sched-hour').value);
            const m = parseInt(document.getElementById('sched-minute').value);
            await fetch(`/api/vehicles/${VID}/charge/schedule`, {method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify({hour:h,minute:m})});
            alert('Schedule set to ' + h + ':' + String(m).padStart(2,'0'));
        }
        </script>"#.to_string()
    };

    base_html("Charge Control", "charge", &content)
}

/// Render the trip history page.
pub fn render_trips_page(trips: &[Trip]) -> String {
    let content = if trips.is_empty() {
        r#"<h1>Trips</h1>
        <div class="empty-state">
            <p>No trips recorded yet</p>
            <p style="font-size: 0.9rem;">Trips will appear here as you drive with recording enabled.</p>
        </div>"#
            .to_string()
    } else {
        let rows: String = trips
            .iter()
            .map(|t| {
                let distance = t
                    .distance_km
                    .map(|d| format!("{d:.1} km"))
                    .unwrap_or_else(|| "—".to_string());
                let consumption = t
                    .consumption_kwh
                    .map(|c| format!("{c:.2} kWh"))
                    .unwrap_or_else(|| "—".to_string());
                let efficiency = match (t.distance_km, t.consumption_kwh) {
                    (Some(d), Some(c)) if d > 0.0 => format!("{:.1} kWh/100km", c / d * 100.0),
                    _ => "—".to_string(),
                };
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    t.start_at.format("%Y-%m-%d %H:%M"),
                    t.end_at.format("%H:%M"),
                    escape_html(&t.vin),
                    distance,
                    consumption,
                    efficiency,
                )
            })
            .collect();

        format!(
            r#"<h1>Trips</h1>
            <div class="card" style="padding: 0; overflow-x: auto;">
                <table>
                    <thead>
                        <tr><th>Start</th><th>End</th><th>VIN</th><th>Distance</th><th>Energy</th><th>Efficiency</th></tr>
                    </thead>
                    <tbody>{rows}</tbody>
                </table>
            </div>"#
        )
    };

    base_html("Trips", "trips", &content)
}

/// Render the electricity pricing settings page.
pub fn render_settings_page(config: &AppConfig) -> String {
    let elec = &config.electricity;
    let night_price = elec
        .night_price_per_kwh
        .map(|p| p.to_string())
        .unwrap_or_default();
    let night_sh = elec
        .night_start_hour
        .map(|h| h.to_string())
        .unwrap_or_default();
    let night_sm = elec
        .night_start_minute
        .map(|m| m.to_string())
        .unwrap_or_default();
    let night_eh = elec
        .night_end_hour
        .map(|h| h.to_string())
        .unwrap_or_default();
    let night_em = elec
        .night_end_minute
        .map(|m| m.to_string())
        .unwrap_or_default();

    let content = format!(
        r#"<h1>Settings</h1>
        <div class="card">
            <div class="card-title">Electricity Pricing</div>
            <div class="form-group">
                <label for="price">Price per kWh ({currency})</label>
                <input type="number" id="price" step="0.01" value="{price}">
            </div>
            <div class="form-group">
                <label for="currency">Currency</label>
                <input type="text" id="currency" value="{currency}">
            </div>
            <h2 style="margin-top: 1.5rem;">Night Rate (optional)</h2>
            <div class="form-group">
                <label for="night-price">Night price per kWh</label>
                <input type="number" id="night-price" step="0.01" value="{night_price}">
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label for="night-sh">Night start hour</label>
                    <input type="number" id="night-sh" min="0" max="23" value="{night_sh}">
                </div>
                <div class="form-group">
                    <label for="night-sm">Night start minute</label>
                    <input type="number" id="night-sm" min="0" max="59" value="{night_sm}">
                </div>
                <div class="form-group">
                    <label for="night-eh">Night end hour</label>
                    <input type="number" id="night-eh" min="0" max="23" value="{night_eh}">
                </div>
                <div class="form-group">
                    <label for="night-em">Night end minute</label>
                    <input type="number" id="night-em" min="0" max="59" value="{night_em}">
                </div>
            </div>
            <button class="btn btn-primary" onclick="saveSettings()">Save Settings</button>
        </div>
        <div class="card">
            <div class="card-title">PSA API</div>
            <div class="stat"><span class="stat-label">Brand</span><span class="stat-value">{brand}</span></div>
            <div class="stat"><span class="stat-label">Client ID</span><span class="stat-value">{client_id_masked}</span></div>
        </div>
        <script>
        async function saveSettings() {{
            const body = {{
                price_per_kwh: parseFloat(document.getElementById('price').value) || 0,
                currency: document.getElementById('currency').value,
                night_price_per_kwh: parseFloat(document.getElementById('night-price').value) || null,
                night_start_hour: parseInt(document.getElementById('night-sh').value) || null,
                night_start_minute: parseInt(document.getElementById('night-sm').value) || null,
                night_end_hour: parseInt(document.getElementById('night-eh').value) || null,
                night_end_minute: parseInt(document.getElementById('night-em').value) || null,
            }};
            const res = await fetch('/api/settings', {{method:'POST', headers:{{'Content-Type':'application/json'}}, body:JSON.stringify(body)}});
            if (res.ok) alert('Settings saved'); else alert('Failed to save settings');
        }}
        </script>"#,
        price = elec.price_per_kwh,
        currency = escape_html(&elec.currency),
        night_price = night_price,
        night_sh = night_sh,
        night_sm = night_sm,
        night_eh = night_eh,
        night_em = night_em,
        brand = escape_html(&config.psa.brand),
        client_id_masked = escape_html(&mask_string(&config.psa.client_id)),
    );

    base_html("Settings", "settings", &content)
}

/// Show only the first 4 characters of a string, replacing the rest with asterisks.
fn mask_string(s: &str) -> String {
    if s.len() <= 4 {
        "****".to_string()
    } else {
        let visible: String = s.chars().take(4).collect();
        format!("{visible}****")
    }
}
