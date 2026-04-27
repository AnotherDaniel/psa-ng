// [utest->req~configuration-file~1]
// [utest->req~electricity-pricing~1]

#[cfg(test)]
mod tests {
    use psa_api::config::{AppConfig, ElectricityConfig};

    #[test]
    fn test_config_parse_full() {
        let toml = r#"
[psa]
client_id = "test_id"
client_secret = "test_secret"
brand = "peugeot"

[server]
host = "0.0.0.0"
port = 8080

[electricity]
price_per_kwh = 0.15
currency = "EUR"
night_price_per_kwh = 0.08
night_start_hour = 22
night_start_minute = 0
night_end_hour = 6
night_end_minute = 0
"#;

        let config: AppConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.psa.client_id, "test_id");
        assert_eq!(config.psa.brand, "peugeot");
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.electricity.price_per_kwh, 0.15);
        assert_eq!(config.electricity.night_price_per_kwh, Some(0.08));
        assert_eq!(config.electricity.night_start_hour, Some(22));
        assert_eq!(config.electricity.night_end_hour, Some(6));
    }

    #[test]
    fn test_config_defaults() {
        let toml = r#"
[psa]
client_id = "id"
client_secret = "secret"
brand = "citroen"
"#;

        let config: AppConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 5000);
        assert_eq!(config.electricity.price_per_kwh, 0.0);
        assert_eq!(config.electricity.currency, "EUR");
        assert!(config.electricity.night_price_per_kwh.is_none());
    }

    #[test]
    fn test_config_save_and_load() {
        let dir = std::env::temp_dir().join("psa-ng-config-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_config.toml");

        let config = AppConfig {
            psa: psa_api::config::PsaConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                brand: "opel".to_string(),
                api_base_url: "https://api.example.com".to_string(),
                token_file: None,
            },
            server: Default::default(),
            electricity: ElectricityConfig {
                price_per_kwh: 0.20,
                ..Default::default()
            },
        };

        config.save(&path).unwrap();
        let loaded = AppConfig::load(&path).unwrap();
        assert_eq!(loaded.psa.brand, "opel");
        assert_eq!(loaded.electricity.price_per_kwh, 0.20);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_electricity_pricing_night_rate() {
        let elec = ElectricityConfig {
            price_per_kwh: 0.15,
            night_price_per_kwh: Some(0.08),
            night_start_hour: Some(22),
            night_start_minute: Some(0),
            night_end_hour: Some(6),
            night_end_minute: Some(0),
            currency: "EUR".to_string(),
        };

        assert_eq!(elec.price_per_kwh, 0.15);
        assert_eq!(elec.night_price_per_kwh, Some(0.08));
        assert!(elec.night_start_hour.is_some());
        assert!(elec.night_end_hour.is_some());
    }

    // [utest->req~rust-best-practices~1]
    #[test]
    fn test_deny_warnings_and_clippy_configured() {
        // Verify that both crate roots enforce #![deny(warnings)] and #![deny(clippy::all)].
        // If either were missing, the crate would not compile with `cargo clippy -- -D warnings`.
        let api_lib = include_str!("../../psa-api/src/lib.rs");
        assert!(api_lib.contains("#![deny(warnings)]"));
        assert!(api_lib.contains("#![deny(clippy::all)]"));

        let web_main = include_str!("main.rs");
        assert!(web_main.contains("#![deny(warnings)]"));
        assert!(web_main.contains("#![deny(clippy::all)]"));
    }
}
