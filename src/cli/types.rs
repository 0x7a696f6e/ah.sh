use clap::{Parser, Subcommand};

use crate::APP_NAME;
use crate::cli::completions;
use crate::provider::{Language, ProviderType};
use crate::session::SessionKey;

const BEFORE_HELP: &str = "

    █████   ██  ██
   ██   ██  ██  ██
   ███████  ██████
   ██   ██  ██  ██
   ██   ██  ██  ██ .sh";

const ABOUT: &str = "Ad-hoc development shell manager powered by Nix";

const AFTER_LONG_HELP: &str = "\x1b[1;4mAliases:\x1b[0m
  ah          ->  ah use
  ah restore  ->  ah session restore
  ah update   ->  ah session update

Use \x1b[1;3mah <COMMAND> --help\x1b[0m for more information about a command.
";

#[derive(Parser)]
#[command(
    name = APP_NAME,
    version,
    about = ABOUT,
    before_help = BEFORE_HELP,
    after_help = AFTER_LONG_HELP,
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(hide = true)]
    pub languages: Option<Vec<Language>>,

    #[arg(hide = true, short, long)]
    pub provider: Option<ProviderType>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List and inspect providers
    Provider {
        #[command(subcommand)]
        command: ProviderCommands,
    },

    /// Manage development sessions
    Session {
        #[command(subcommand)]
        command: SessionCommands,
    },

    /// Restore a session by index, ID, or show history for current directory
    #[command(hide = true)]
    Restore {
        /// Session index (1, 2, ...) or ID (8 hex chars). Shows history if not specified
        #[arg(add = completions::make_session_key_completer())]
        key: Option<SessionKey>,
    },

    /// Update session dependencies
    #[command(hide = true)]
    Update {
        /// Session index (1, 2, ...) or ID (8 hex chars). Uses current session if not specified
        #[arg(add = completions::make_session_key_completer())]
        session: Option<SessionKey>,
    },

    /// Create and enter a development environment
    Use {
        /// Languages to enable (e.g., rust go nodejs)
        #[arg(required = true, num_args = 1.., add = completions::make_language_completer())]
        languages: Vec<Language>,

        /// Which provider to use
        #[arg(short, long)]
        provider: Option<ProviderType>,
    },

    /// Print completion script by shell
    Completion { shell: clap_complete::Shell },
}

#[derive(Subcommand, Debug)]
pub enum SessionCommands {
    /// Delete all sessions
    Clear,

    /// List all sessions
    List,

    /// Delete one or more sessions by index or ID
    Remove {
        /// Session index(es) or ID(s) (8 hex chars)
        #[arg(required = true, num_args = 1.., add = completions::make_session_key_completer())]
        keys: Vec<SessionKey>,
    },

    /// Restore a session by index, ID, or show history for current directory
    Restore {
        /// Session index (1, 2, ...) or ID (8 hex chars). Shows history if not specified
        #[arg(add = completions::make_session_key_completer())]
        key: Option<SessionKey>,
    },

    /// Update session dependencies
    Update {
        /// Session index (1, 2, ...) or ID (8 hex chars). Uses current session if not specified
        #[arg(add = completions::make_session_key_completer())]
        session: Option<SessionKey>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProviderCommands {
    /// List all available providers
    List,

    /// Show supported languages for a provider
    Show { provider: ProviderType },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::SessionKey;
    use clap::Parser;
    use clap_complete::Shell;

    #[test]
    fn implicit_use_from_top_level_languages() {
        let cli = Cli::try_parse_from(["ah", "rust", "go"]).unwrap();
        assert!(cli.command.is_none());
        assert_eq!(
            cli.languages,
            Some(vec!["rust".to_string(), "go".to_string()])
        );
        assert_eq!(cli.provider, None);
    }

    #[test]
    fn implicit_use_with_top_level_provider_flag() {
        let cli = Cli::try_parse_from(["ah", "-p", "devenv", "rust"]).unwrap();
        assert!(cli.command.is_none());
        assert_eq!(cli.languages, Some(vec!["rust".to_string()]));
        assert_eq!(cli.provider, Some(ProviderType::Devenv));
    }

    #[test]
    fn explicit_use_subcommand() {
        let cli = Cli::try_parse_from(["ah", "use", "rust", "go", "--provider", "dev-templates"])
            .unwrap();
        match cli.command {
            Some(Commands::Use {
                languages,
                provider,
            }) => {
                assert_eq!(languages, vec!["rust".to_string(), "go".to_string()]);
                assert_eq!(provider, Some(ProviderType::DevTemplates));
            }
            other => panic!("expected Use, got {other:?}"),
        }
    }

    #[test]
    fn explicit_use_short_provider_flag() {
        let cli = Cli::try_parse_from(["ah", "use", "rust", "-p", "devenv"]).unwrap();
        match cli.command {
            Some(Commands::Use { provider, .. }) => {
                assert_eq!(provider, Some(ProviderType::Devenv));
            }
            other => panic!("expected Use, got {other:?}"),
        }
    }

    #[test]
    fn use_requires_at_least_one_language() {
        assert!(Cli::try_parse_from(["ah", "use"]).is_err());
    }

    #[test]
    fn session_list() {
        let cli = Cli::try_parse_from(["ah", "session", "list"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Session {
                command: SessionCommands::List
            })
        ));
    }

    #[test]
    fn session_clear() {
        let cli = Cli::try_parse_from(["ah", "session", "clear"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Session {
                command: SessionCommands::Clear
            })
        ));
    }

    #[test]
    fn session_remove_accepts_mixed_keys() {
        let cli = Cli::try_parse_from(["ah", "session", "remove", "1", "2", "a3f8c2d1"]).unwrap();
        match cli.command {
            Some(Commands::Session {
                command: SessionCommands::Remove { keys },
            }) => assert_eq!(
                keys,
                vec![
                    SessionKey::Index(1),
                    SessionKey::Index(2),
                    SessionKey::Id("a3f8c2d1".to_string()),
                ]
            ),
            other => panic!("expected Remove, got {other:?}"),
        }
    }

    #[test]
    fn session_remove_rejects_zero_index() {
        let cli = Cli::try_parse_from(["ah", "session", "remove", "0"]);
        assert!(
            cli.is_err(),
            "zero index should be rejected by SessionKey parser"
        );
    }

    #[test]
    fn session_restore_without_key() {
        let cli = Cli::try_parse_from(["ah", "session", "restore"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Session {
                command: SessionCommands::Restore { key: None }
            })
        ));
    }

    #[test]
    fn session_update_with_id() {
        let cli = Cli::try_parse_from(["ah", "session", "update", "deadbeef"]).unwrap();
        match cli.command {
            Some(Commands::Session {
                command:
                    SessionCommands::Update {
                        session: Some(SessionKey::Id(id)),
                    },
            }) => assert_eq!(id, "deadbeef"),
            other => panic!("expected Update with Id, got {other:?}"),
        }
    }

    #[test]
    fn provider_list() {
        let cli = Cli::try_parse_from(["ah", "provider", "list"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Provider {
                command: ProviderCommands::List
            })
        ));
    }

    #[test]
    fn provider_show_accepts_known_provider() {
        let cli = Cli::try_parse_from(["ah", "provider", "show", "devenv"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Provider {
                command: ProviderCommands::Show {
                    provider: ProviderType::Devenv
                }
            })
        ));
    }

    #[test]
    fn provider_show_rejects_unknown_value() {
        assert!(Cli::try_parse_from(["ah", "provider", "show", "nope"]).is_err());
    }

    #[test]
    fn completion_subcommand_for_bash() {
        let cli = Cli::try_parse_from(["ah", "completion", "bash"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::Completion { shell: Shell::Bash })
        ));
    }

    #[test]
    fn hidden_restore_alias_with_index() {
        let cli = Cli::try_parse_from(["ah", "restore", "3"]).unwrap();
        match cli.command {
            Some(Commands::Restore {
                key: Some(SessionKey::Index(3)),
            }) => {}
            other => panic!("expected Restore Index(3), got {other:?}"),
        }
    }

    #[test]
    fn hidden_update_alias_with_id() {
        let cli = Cli::try_parse_from(["ah", "update", "a3f8c2d1"]).unwrap();
        match cli.command {
            Some(Commands::Update {
                session: Some(SessionKey::Id(id)),
            }) => assert_eq!(id, "a3f8c2d1"),
            other => panic!("expected Update Id, got {other:?}"),
        }
    }

    #[test]
    fn unknown_session_subcommand_fails() {
        assert!(Cli::try_parse_from(["ah", "session", "bogus"]).is_err());
    }

    #[test]
    fn unknown_flag_under_use_fails() {
        assert!(Cli::try_parse_from(["ah", "use", "rust", "--bogus"]).is_err());
    }
}
