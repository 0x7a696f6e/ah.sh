use std::collections::HashSet;
use std::time::SystemTime;

use anyhow::Result;
use tracing_attributes::instrument;

use crate::path;
use crate::provider::{Language, ProviderType, to_supported_languages};

mod storage;
mod types;
pub use storage::*;
pub use types::*;

pub fn generate_id(provider: ProviderType, languages: &[String]) -> String {
    let input = format!("{}:{}", provider, languages.join(","));
    let digest = blake3::hash(input.as_bytes());
    digest.to_hex().to_string()[..SESSION_ID_LEN].to_string()
}

#[instrument(ret, err)]
pub fn find_session(provider: ProviderType, languages: &[Language]) -> Result<Option<Session>> {
    let supported_languages = to_supported_languages(provider, languages)?;
    let session_id = generate_id(provider, &supported_languages);
    let session_dir = path::cache::sessions::get_dir().join(&session_id);
    if !session_dir.is_dir() {
        return Ok(None);
    }
    try_session_by_id(&session_id).map(Some)
}

pub fn remove_sessions(keys: &[SessionKey]) -> Result<Option<SessionRemoveResult>> {
    if keys.is_empty() {
        return Ok(None);
    }

    let mut removed_ids = Vec::new();
    let mut missing_keys = Vec::new();
    let mut deduped_session_ids = HashSet::new();

    for key in keys {
        match try_session_by_key(key) {
            Ok(session) => {
                if deduped_session_ids.insert(session.id.clone()) {
                    let session_id = session.id.clone();
                    if remove_session(&session_id)? {
                        removed_ids.push(session_id);
                    } else {
                        missing_keys.push(session_id);
                    }
                }
            }
            Err(_) => {
                missing_keys.push(key.to_string());
            }
        }
    }

    Ok(Some(SessionRemoveResult {
        removed_ids,
        missing_keys,
    }))
}

#[instrument(ret, err)]
pub fn create_session(provider: ProviderType, languages: Vec<Language>) -> Result<Session> {
    let supported_languages = to_supported_languages(provider, &languages)?;

    let session_id = generate_id(provider, &supported_languages);

    let session = Session {
        id: session_id.clone(),
        provider,
        languages: supported_languages,
        last_used_at: SystemTime::now(),
        last_updated_at: SystemTime::now(),
    };

    save_session(&session)?;

    Ok(session)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_id_is_deterministic() {
        let id1 = generate_id(ProviderType::Devenv, &["rust".to_string()]);
        let id2 = generate_id(ProviderType::Devenv, &["rust".to_string()]);
        assert_eq!(id1, id2);
    }

    #[test]
    fn generate_id_has_session_id_length() {
        let id = generate_id(
            ProviderType::Devenv,
            &["rust".to_string(), "go".to_string()],
        );
        assert_eq!(id.len(), SESSION_ID_LEN);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn different_providers_produce_different_ids() {
        let devenv = generate_id(ProviderType::Devenv, &["rust".to_string()]);
        let templates = generate_id(ProviderType::DevTemplates, &["rust".to_string()]);
        assert_ne!(devenv, templates);
    }

    #[test]
    fn different_languages_produce_different_ids() {
        let a = generate_id(ProviderType::Devenv, &["rust".to_string()]);
        let b = generate_id(ProviderType::Devenv, &["go".to_string()]);
        assert_ne!(a, b);
    }

    #[test]
    fn language_order_affects_id() {
        let a = generate_id(
            ProviderType::Devenv,
            &["rust".to_string(), "go".to_string()],
        );
        let b = generate_id(
            ProviderType::Devenv,
            &["go".to_string(), "rust".to_string()],
        );
        assert_ne!(a, b);
    }

    #[test]
    fn empty_languages_yields_a_valid_id() {
        let id = generate_id(ProviderType::Devenv, &[]);
        assert_eq!(id.len(), SESSION_ID_LEN);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
