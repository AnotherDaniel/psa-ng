// [impl->req~status-history~1]
// [impl->req~trip-recording~1]
// [impl->req~charging-session-recording~1]

//! SQLite persistence layer for vehicle status history, trips, and charging sessions.

use chrono::{DateTime, Utc};
use psa_api::models::{ChargingSession, Trip};
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::Mutex;

/// A point-in-time vehicle status record ready for database insertion.
#[allow(dead_code)] // used by tests; called from polling loop once req~status-polling~1 is implemented
pub struct StatusSnapshot<'a> {
    pub vin: &'a str,
    pub timestamp: &'a DateTime<Utc>,
    pub battery_level: Option<f64>,
    pub charging_status: Option<&'a str>,
    pub mileage_km: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub autonomy_km: Option<f64>,
    pub raw_json: Option<&'a str>,
}

/// SQLite database handle protected by a mutex for thread-safe access.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Open (or create) a SQLite database at `path` and initialize the schema.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_tables()?;
        Ok(db)
    }

    /// Create tables and indexes if they do not already exist.
    fn init_tables(&self) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("DB lock poisoned");
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS status_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                vin TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                battery_level REAL,
                charging_status TEXT,
                mileage_km REAL,
                latitude REAL,
                longitude REAL,
                autonomy_km REAL,
                raw_json TEXT
            );

            CREATE TABLE IF NOT EXISTS trips (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                vin TEXT NOT NULL,
                start_at TEXT NOT NULL,
                end_at TEXT NOT NULL,
                start_lat REAL,
                start_lon REAL,
                end_lat REAL,
                end_lon REAL,
                distance_km REAL,
                consumption_kwh REAL
            );

            CREATE TABLE IF NOT EXISTS charging_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                vin TEXT NOT NULL,
                start_at TEXT NOT NULL,
                end_at TEXT,
                start_level REAL,
                end_level REAL,
                energy_kwh REAL,
                cost REAL
            );

            CREATE INDEX IF NOT EXISTS idx_status_vin_ts ON status_history(vin, timestamp);
            CREATE INDEX IF NOT EXISTS idx_trips_vin ON trips(vin, start_at);
            CREATE INDEX IF NOT EXISTS idx_charging_vin ON charging_sessions(vin, start_at);
            ",
        )?;
        Ok(())
    }

    /// Insert a vehicle status snapshot into the history table.
    #[allow(dead_code)] // pending req~status-polling~1
    pub fn insert_status_snapshot(&self, snap: &StatusSnapshot<'_>) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("DB lock poisoned");
        conn.execute(
            "INSERT INTO status_history (vin, timestamp, battery_level, charging_status, mileage_km, latitude, longitude, autonomy_km, raw_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                snap.vin,
                snap.timestamp.to_rfc3339(),
                snap.battery_level,
                snap.charging_status,
                snap.mileage_km,
                snap.latitude,
                snap.longitude,
                snap.autonomy_km,
                snap.raw_json,
            ],
        )?;
        Ok(())
    }

    /// Insert a completed trip record.
    #[allow(dead_code)] // pending req~status-polling~1
    pub fn insert_trip(&self, trip: &Trip) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("DB lock poisoned");
        conn.execute(
            "INSERT INTO trips (vin, start_at, end_at, start_lat, start_lon, end_lat, end_lon, distance_km, consumption_kwh)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                trip.vin,
                trip.start_at.to_rfc3339(),
                trip.end_at.to_rfc3339(),
                trip.start_lat,
                trip.start_lon,
                trip.end_lat,
                trip.end_lon,
                trip.distance_km,
                trip.consumption_kwh,
            ],
        )?;
        Ok(())
    }

    /// Retrieve trips, optionally filtered by VIN, ordered newest-first.
    pub fn get_trips(&self, vin: Option<&str>) -> rusqlite::Result<Vec<Trip>> {
        let conn = self.conn.lock().expect("DB lock poisoned");
        let mut trips = Vec::new();

        let (query, params_vec): (&str, Vec<Box<dyn rusqlite::types::ToSql>>) = match vin {
            Some(v) => (
                "SELECT id, vin, start_at, end_at, start_lat, start_lon, end_lat, end_lon, distance_km, consumption_kwh FROM trips WHERE vin = ?1 ORDER BY start_at DESC",
                vec![Box::new(v.to_string())],
            ),
            None => (
                "SELECT id, vin, start_at, end_at, start_lat, start_lon, end_lat, end_lon, distance_km, consumption_kwh FROM trips ORDER BY start_at DESC",
                vec![],
            ),
        };

        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            let start_str: String = row.get(2)?;
            let end_str: String = row.get(3)?;
            Ok(Trip {
                id: row.get(0)?,
                vin: row.get(1)?,
                start_at: DateTime::parse_from_rfc3339(&start_str)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                end_at: DateTime::parse_from_rfc3339(&end_str)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                start_lat: row.get(4)?,
                start_lon: row.get(5)?,
                end_lat: row.get(6)?,
                end_lon: row.get(7)?,
                distance_km: row.get(8)?,
                consumption_kwh: row.get(9)?,
            })
        })?;

        for row in rows {
            trips.push(row?);
        }
        Ok(trips)
    }

    /// Insert a charging session record.
    #[allow(dead_code)] // pending req~status-polling~1
    pub fn insert_charging_session(&self, session: &ChargingSession) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("DB lock poisoned");
        conn.execute(
            "INSERT INTO charging_sessions (vin, start_at, end_at, start_level, end_level, energy_kwh, cost)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                session.vin,
                session.start_at.to_rfc3339(),
                session.end_at.map(|d| d.to_rfc3339()),
                session.start_level,
                session.end_level,
                session.energy_kwh,
                session.cost,
            ],
        )?;
        Ok(())
    }

    /// Retrieve charging sessions, optionally filtered by VIN, ordered newest-first.
    pub fn get_charging_sessions(
        &self,
        vin: Option<&str>,
    ) -> rusqlite::Result<Vec<ChargingSession>> {
        let conn = self.conn.lock().expect("DB lock poisoned");
        let mut sessions = Vec::new();

        let (query, params_vec): (&str, Vec<Box<dyn rusqlite::types::ToSql>>) = match vin {
            Some(v) => (
                "SELECT id, vin, start_at, end_at, start_level, end_level, energy_kwh, cost FROM charging_sessions WHERE vin = ?1 ORDER BY start_at DESC",
                vec![Box::new(v.to_string())],
            ),
            None => (
                "SELECT id, vin, start_at, end_at, start_level, end_level, energy_kwh, cost FROM charging_sessions ORDER BY start_at DESC",
                vec![],
            ),
        };

        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            let start_str: String = row.get(2)?;
            let end_str: Option<String> = row.get(3)?;
            Ok(ChargingSession {
                id: row.get(0)?,
                vin: row.get(1)?,
                start_at: DateTime::parse_from_rfc3339(&start_str)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                end_at: end_str.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|d| d.with_timezone(&Utc))
                }),
                start_level: row.get(4)?,
                end_level: row.get(5)?,
                energy_kwh: row.get(6)?,
                cost: row.get(7)?,
            })
        })?;

        for row in rows {
            sessions.push(row?);
        }
        Ok(sessions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn temp_db() -> Database {
        Database::open(Path::new(":memory:")).unwrap()
    }

    // [utest->req~status-history~1]
    #[test]
    fn test_insert_and_query_status() {
        let db = temp_db();
        let now = Utc::now();
        db.insert_status_snapshot(&StatusSnapshot {
            vin: "VIN123",
            timestamp: &now,
            battery_level: Some(75.0),
            charging_status: Some("Disconnected"),
            mileage_km: Some(15000.0),
            latitude: Some(48.85),
            longitude: Some(2.35),
            autonomy_km: Some(220.0),
            raw_json: None,
        })
        .unwrap();
    }

    // [utest->req~trip-recording~1]
    #[test]
    fn test_insert_and_get_trips() {
        let db = temp_db();
        let now = Utc::now();
        let trip = Trip {
            id: 0,
            vin: "VIN123".to_string(),
            start_at: now - Duration::hours(1),
            end_at: now,
            start_lat: Some(48.85),
            start_lon: Some(2.35),
            end_lat: Some(48.90),
            end_lon: Some(2.40),
            distance_km: Some(12.5),
            consumption_kwh: Some(2.3),
        };
        db.insert_trip(&trip).unwrap();

        let trips = db.get_trips(Some("VIN123")).unwrap();
        assert_eq!(trips.len(), 1);
        assert_eq!(trips[0].vin, "VIN123");
        assert_eq!(trips[0].distance_km, Some(12.5));
    }

    // [utest->req~charging-session-recording~1]
    #[test]
    fn test_insert_and_get_charging_sessions() {
        let db = temp_db();
        let now = Utc::now();
        let session = ChargingSession {
            id: 0,
            vin: "VIN123".to_string(),
            start_at: now - Duration::hours(2),
            end_at: Some(now),
            start_level: Some(30.0),
            end_level: Some(80.0),
            energy_kwh: Some(25.0),
            cost: Some(3.75),
        };
        db.insert_charging_session(&session).unwrap();

        let sessions = db.get_charging_sessions(Some("VIN123")).unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].energy_kwh, Some(25.0));
        assert_eq!(sessions[0].cost, Some(3.75));
    }
}
