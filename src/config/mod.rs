//! Loads and parses the TOML configuration that defines the app carousel:
//! entries, icons, and launch actions.

use std::fmt;
use std::fs;
use std::path::Path;

use serde::Deserialize;

/// Config path used when no explicit path is given, relative to the
/// current working directory.
pub const DEFAULT_CONFIG_PATH: &str = "config/hub.toml";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub apps: Vec<AppEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppEntry {
    pub name: String,
    /// Icon filename, resolved relative to `assets/icons/` (e.g. "plex.png").
    pub icon: Option<String>,
    pub action: AppAction,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AppAction {
    Url {
        url: String,
        browser: Option<String>,
    },
    Command {
        command: String,
        args: Option<Vec<String>>,
    },
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "failed to read config file: {e}"),
            ConfigError::Parse(e) => write!(f, "failed to parse config file: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e)
    }
}

impl Config {
    /// Strictly load and parse a config file at `path`, failing on any I/O
    /// or parse error.
    pub fn load(path: &Path) -> Result<Config, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load from `explicit` if given, else [`DEFAULT_CONFIG_PATH`]. Never
    /// fails: falls back to a small set of built-in placeholder entries
    /// (logging a warning) so the hub always has something to show.
    pub fn load_default(explicit: Option<&Path>) -> Config {
        let path = explicit.unwrap_or_else(|| Path::new(DEFAULT_CONFIG_PATH));
        match Config::load(path) {
            Ok(config) => config,
            Err(err) => {
                eprintln!(
                    "warning: could not load config from {}: {err}; using built-in defaults",
                    path.display()
                );
                Config::fallback()
            }
        }
    }

    /// A minimal built-in config, used when no config file can be loaded.
    pub fn fallback() -> Config {
        Config {
            apps: vec![AppEntry {
                name: "Example".to_string(),
                icon: None,
                action: AppAction::Url {
                    url: "https://example.com".to_string(),
                    browser: None,
                },
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_url_and_command_actions() {
        let toml = r#"
            [[apps]]
            name = "Plex"
            icon = "plex.png"
            [apps.action]
            type = "url"
            url = "http://plex.local"

            [[apps]]
            name = "Kodi"
            icon = "kodi.png"
            [apps.action]
            type = "command"
            command = "kodi"
        "#;

        let config: Config = toml::from_str(toml).expect("valid config should parse");
        assert_eq!(config.apps.len(), 2);

        assert_eq!(config.apps[0].name, "Plex");
        assert_eq!(config.apps[0].icon.as_deref(), Some("plex.png"));
        match &config.apps[0].action {
            AppAction::Url { url, browser } => {
                assert_eq!(url, "http://plex.local");
                assert_eq!(*browser, None);
            }
            other => panic!("expected Url action, got {other:?}"),
        }

        assert_eq!(config.apps[1].name, "Kodi");
        match &config.apps[1].action {
            AppAction::Command { command, args } => {
                assert_eq!(command, "kodi");
                assert_eq!(*args, None);
            }
            other => panic!("expected Command action, got {other:?}"),
        }
    }

    #[test]
    fn parses_optional_browser_and_args_fields() {
        let toml = r#"
            [[apps]]
            name = "YouTube"
            [apps.action]
            type = "url"
            url = "https://youtube.com"
            browser = "firefox"

            [[apps]]
            name = "Sleep"
            [apps.action]
            type = "command"
            command = "sleep"
            args = ["5"]
        "#;

        let config: Config = toml::from_str(toml).expect("valid config should parse");

        match &config.apps[0].action {
            AppAction::Url { browser, .. } => assert_eq!(browser.as_deref(), Some("firefox")),
            other => panic!("expected Url action, got {other:?}"),
        }
        match &config.apps[1].action {
            AppAction::Command { args, .. } => {
                assert_eq!(args.as_deref(), Some(&["5".to_string()][..]))
            }
            other => panic!("expected Command action, got {other:?}"),
        }
    }

    #[test]
    fn rejects_malformed_toml_without_panicking() {
        let toml = "this is not valid toml [[[";
        let result: Result<Config, toml::de::Error> = toml::from_str(toml);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_action_type() {
        let toml = r#"
            [[apps]]
            name = "Broken"
            [apps.action]
            type = "not_a_real_type"
        "#;
        let result: Result<Config, toml::de::Error> = toml::from_str(toml);
        assert!(result.is_err());
    }

    #[test]
    fn load_reports_error_for_missing_file() {
        let result = Config::load(Path::new("this/path/does/not/exist.toml"));
        assert!(matches!(result, Err(ConfigError::Io(_))));
    }

    #[test]
    fn load_default_falls_back_when_file_missing() {
        let config = Config::load_default(Some(Path::new("this/path/does/not/exist.toml")));
        assert!(!config.apps.is_empty());
    }

    #[test]
    fn shipped_hub_toml_parses_and_has_six_entries() {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config/hub.toml"));
        let config = Config::load(path).expect("config/hub.toml should parse");
        assert_eq!(config.apps.len(), 6);
    }
}
