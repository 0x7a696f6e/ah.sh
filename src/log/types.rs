#[derive(
    Clone,
    Copy,
    Debug,
    schemars::JsonSchema,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    OFF,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parses_all_lowercase_variants() {
        for s in ["trace", "debug", "info", "warn", "error", "off"] {
            let parsed = LogLevel::from_str(s).unwrap();
            assert_eq!(parsed.to_string(), s);
        }
    }

    #[test]
    fn display_and_from_str_roundtrip() {
        for s in ["trace", "debug", "info", "warn", "error", "off"] {
            let parsed = LogLevel::from_str(s).unwrap();
            assert_eq!(parsed.to_string(), s);
        }
    }

    #[test]
    fn rejects_uppercase_and_unknown() {
        assert!(LogLevel::from_str("TRACE").is_err());
        assert!(LogLevel::from_str("Info").is_err());
        assert!(LogLevel::from_str("verbose").is_err());
        assert!(LogLevel::from_str("").is_err());
    }

    #[test]
    fn serde_uses_lowercase() {
        for s in ["trace", "debug", "info", "warn", "error", "off"] {
            let json = serde_json::to_string(&LogLevel::from_str(s).unwrap()).unwrap();
            assert_eq!(json, format!("\"{s}\""));
        }
        assert!(serde_json::from_str::<LogLevel>("\"INFO\"").is_err());
    }
}
