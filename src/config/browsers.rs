//! Named browser launch profiles, configured separately from the app
//! carousel so launch flags (kiosk mode, ad-block extensions, etc.) can be
//! retuned per-profile without touching `hub.toml`.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use super::ConfigError;

/// Browser profiles config path used when no explicit path is given,
/// relative to the current working directory.
pub const DEFAULT_BROWSER_CONFIG_PATH: &str = "config/browser.toml";
/// Name of the single browser profile shipped when no browser config file
/// can be loaded.
const FALLBACK_BROWSER_NAME: &str = "chromium";

#[derive(Debug, Clone, Deserialize)]
pub struct Browsers {
    /// Name of the profile used when an `AppAction::Url` doesn't specify one.
    pub default: String,
    pub browsers: HashMap<String, BrowserEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BrowserEntry {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl Browsers {
    pub fn load(path: &Path) -> Result<Browsers, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let browsers = toml::from_str(&contents)?;
        Ok(browsers)
    }

    pub fn load_default(explicit: Option<&Path>) -> Browsers {
        let path = explicit.unwrap_or_else(|| Path::new(DEFAULT_BROWSER_CONFIG_PATH));
        match Browsers::load(path) {
            Ok(browsers) => browsers,
            Err(err) => {
                eprintln!(
                    "warning: could not load browser config from {}: {err}; using built-in defaults",
                    path.display()
                );
                Browsers::fallback()
            }
        }
    }

    /// A minimal built-in browser profile, used when no browser config file
    /// can be loaded.
    pub fn fallback() -> Browsers {
        let mut browsers = HashMap::new();
        browsers.insert(
            FALLBACK_BROWSER_NAME.to_string(),
            BrowserEntry {
                command: FALLBACK_BROWSER_NAME.to_string(),
                args: vec!["--kiosk".to_string()],
            },
        );
        Browsers {
            default: FALLBACK_BROWSER_NAME.to_string(),
            browsers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_named_browsers_and_default() {
        let toml = r#"
            default = "chromium"

            [browsers.chromium]
            command = "chromium"
            args = ["--kiosk"]

            [browsers.firefox]
            command = "firefox"
        "#;

        let browsers: Browsers = toml::from_str(toml).expect("valid browser config should parse");
        assert_eq!(browsers.default, "chromium");

        let chromium = &browsers.browsers["chromium"];
        assert_eq!(chromium.command, "chromium");
        assert_eq!(chromium.args, vec!["--kiosk".to_string()]);

        // `args` is optional and defaults to empty.
        let firefox = &browsers.browsers["firefox"];
        assert_eq!(firefox.command, "firefox");
        assert!(firefox.args.is_empty());
    }

    #[test]
    fn load_reports_error_for_missing_file() {
        let result = Browsers::load(Path::new("this/path/does/not/exist.toml"));
        assert!(matches!(result, Err(ConfigError::Io(_))));
    }

    #[test]
    fn load_default_falls_back_when_file_missing() {
        let browsers = Browsers::load_default(Some(Path::new("this/path/does/not/exist.toml")));
        assert!(browsers.browsers.contains_key(&browsers.default));
    }

    #[test]
    fn shipped_browser_toml_parses_and_has_default_entry() {
        let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config/browser.toml"));
        let browsers = Browsers::load(path).expect("config/browser.toml should parse");
        assert!(browsers.browsers.contains_key(&browsers.default));
    }
}
