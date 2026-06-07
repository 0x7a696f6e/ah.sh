use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::path;
use crate::provider::ProviderType;

pub const SESSION_ID_LEN: usize = 8;
pub const HISTORY_LIMIT: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub provider: ProviderType,
    pub languages: Vec<String>,
    pub last_used_at: SystemTime,
    pub last_updated_at: SystemTime,
}

impl Session {
    pub fn get_dir(&self) -> PathBuf {
        path::cache::sessions::get_dir().join(&self.id)
    }
}

pub struct SessionRemoveResult {
    pub removed_ids: Vec<String>,
    pub missing_keys: Vec<String>,
}

#[derive(strum::Display, Debug, Clone, PartialEq, Eq)]
pub enum SessionKey {
    #[strum(transparent)]
    Index(usize),
    #[strum(transparent)]
    Id(String),
}

impl FromStr for SessionKey {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        if input.is_empty() {
            anyhow::bail!("session target cannot be empty");
        }

        if input.chars().all(|c| c.is_ascii_digit()) {
            let index = input.parse::<usize>()?;
            if index == 0 {
                anyhow::bail!("session index must be greater than 0");
            }
            return Ok(SessionKey::Index(index));
        }

        if input.len() != 8 || !input.chars().all(|c| c.is_ascii_hexdigit()) {
            anyhow::bail!("session id must be exactly 8 hexadecimal characters");
        }

        Ok(SessionKey::Id(input.to_lowercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_string_as_index() {
        let key = "1".parse::<SessionKey>().unwrap();
        assert_eq!(key, SessionKey::Index(1));

        let key = "42".parse::<SessionKey>().unwrap();
        assert_eq!(key, SessionKey::Index(42));
    }

    #[test]
    fn parses_eight_hex_chars_as_id() {
        let key = "a3f8c2d1".parse::<SessionKey>().unwrap();
        assert_eq!(key, SessionKey::Id("a3f8c2d1".to_string()));

        let key = "ABCDEF01".parse::<SessionKey>().unwrap();
        assert_eq!(key, SessionKey::Id("abcdef01".to_string()));
    }

    #[test]
    fn empty_string_is_rejected() {
        let err = "".parse::<SessionKey>().unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn zero_index_is_rejected() {
        let err = "0".parse::<SessionKey>().unwrap_err();
        assert!(err.to_string().contains("greater than 0"));
    }

    #[test]
    fn wrong_length_id_is_rejected() {
        for input in ["abc", "abcdef0", "abcdef012", "abcdef0123"] {
            let err = input.parse::<SessionKey>().unwrap_err();
            assert!(
                err.to_string().contains("8 hexadecimal"),
                "unexpected error for {input:?}: {err}"
            );
        }
    }

    #[test]
    fn non_hex_chars_in_id_are_rejected() {
        for input in ["zzzzzzzz", "abcdefg1", "1234567g"] {
            let err = input.parse::<SessionKey>().unwrap_err();
            assert!(
                err.to_string().contains("8 hexadecimal"),
                "unexpected error for {input:?}: {err}"
            );
        }
    }

    #[test]
    fn display_uses_inner_value() {
        assert_eq!(SessionKey::Index(7).to_string(), "7");
        assert_eq!(
            SessionKey::Id("deadbeef".to_string()).to_string(),
            "deadbeef"
        );
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    use crate::provider::ProviderType;
    use std::time::SystemTime;

    fn sample_session() -> Session {
        Session {
            id: "deadbeef".into(),
            provider: ProviderType::Devenv,
            languages: vec!["rust".into(), "go".into()],
            last_used_at: SystemTime::UNIX_EPOCH,
            last_updated_at: SystemTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn session_roundtrips_through_json() {
        let original = sample_session();
        let json = serde_json::to_string(&original).unwrap();
        let restored: Session = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.id, original.id);
        assert_eq!(restored.provider, original.provider);
        assert_eq!(restored.languages, original.languages);
        assert_eq!(restored.last_used_at, original.last_used_at);
        assert_eq!(restored.last_updated_at, original.last_updated_at);
    }

    #[test]
    fn session_json_uses_kebab_case_provider() {
        let session = Session {
            id: "abcd1234".into(),
            provider: ProviderType::DevTemplates,
            languages: vec!["python".into()],
            last_used_at: SystemTime::UNIX_EPOCH,
            last_updated_at: SystemTime::UNIX_EPOCH,
        };
        let pretty = serde_json::to_string_pretty(&session).unwrap();
        assert!(pretty.contains(r#""id": "abcd1234""#));
        assert!(pretty.contains(r#""provider": "dev-templates""#));
        assert!(pretty.contains(r#""languages": ["#));
    }

    #[test]
    fn session_rejects_json_missing_required_field() {
        let incomplete = r#"{"id":"abc","provider":"devenv","languages":[]}"#;
        let err = serde_json::from_str::<Session>(incomplete).unwrap_err();
        assert!(err.to_string().contains("missing field"));
    }
}
