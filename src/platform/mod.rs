//! Platform abstraction for launching applications and OS-specific actions
//! (shutdown, reboot). Keeps `app`/`ui` portable across desktop and Pi
//! targets. Populated in a later milestone.
//!

use std::fmt;

use crate::config::browsers::{BrowserEntry, Browsers};
use log::{debug, info};

#[derive(Debug)]
pub enum LaunchError {
    Spawn(std::io::Error),
    NoDefaultBrowser,
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LaunchError::Spawn(err) => write!(f, "failed to spawn process: {err}"),
            LaunchError::NoDefaultBrowser => write!(f, "no default browser configured"),
        }
    }
}

impl std::error::Error for LaunchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LaunchError::Spawn(err) => Some(err),
            LaunchError::NoDefaultBrowser => None,
        }
    }
}

pub async fn launch_command(
    command: &String,
    args: Option<Vec<String>>,
) -> Result<(), LaunchError> {
    let args = args.unwrap_or(vec![]);
    info!("Launching command: {command} {args:#?}");
    let status = tokio::process::Command::new(command)
        .args(args)
        .status()
        .await
        .map_err(LaunchError::Spawn)?;
    debug!("command process exited with status: {status}");
    Ok(())
}

pub async fn launch_url(
    url: &String,
    browser: &Option<String>,
    browsers: &Browsers,
) -> Result<(), LaunchError> {
    let browser = resolve_browser(browser, browsers)?;
    info!(
        "Launching url: {} {:?} {url}",
        browser.command, browser.args
    );
    let status = tokio::process::Command::new(&browser.command)
        .args(&browser.args)
        .arg(url)
        .status()
        .await
        .map_err(LaunchError::Spawn)?;

    debug!("url process exited with status: {status}");
    Ok(())
}

/// Picks the named browser profile, falling back to the configured default
/// when no name is given or the named profile doesn't exist.
fn resolve_browser<'a>(
    browser: &Option<String>,
    browsers: &'a Browsers,
) -> Result<&'a BrowserEntry, LaunchError> {
    browser
        .as_ref()
        .and_then(|name| browsers.browsers.get(name))
        .or_else(|| browsers.browsers.get(&browsers.default))
        .ok_or(LaunchError::NoDefaultBrowser)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn browsers_with(default: &str, entries: &[(&str, &str)]) -> Browsers {
        let browsers = entries
            .iter()
            .map(|(name, command)| {
                (
                    name.to_string(),
                    BrowserEntry {
                        command: command.to_string(),
                        args: vec![],
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        Browsers {
            default: default.to_string(),
            browsers,
        }
    }

    #[test]
    fn resolve_browser_uses_named_browser_when_present() {
        let browsers = browsers_with(
            "chromium",
            &[("chromium", "chromium"), ("firefox", "firefox")],
        );
        let resolved = resolve_browser(&Some("firefox".to_string()), &browsers).unwrap();
        assert_eq!(resolved.command, "firefox");
    }

    #[test]
    fn resolve_browser_falls_back_to_default_when_none_given() {
        let browsers = browsers_with("chromium", &[("chromium", "chromium")]);
        let resolved = resolve_browser(&None, &browsers).unwrap();
        assert_eq!(resolved.command, "chromium");
    }

    #[test]
    fn resolve_browser_falls_back_to_default_when_named_browser_unknown() {
        let browsers = browsers_with("chromium", &[("chromium", "chromium")]);
        let resolved = resolve_browser(&Some("does-not-exist".to_string()), &browsers).unwrap();
        assert_eq!(resolved.command, "chromium");
    }

    #[test]
    fn resolve_browser_errors_when_default_is_also_missing() {
        let browsers = browsers_with("chromium", &[]);
        let result = resolve_browser(&None, &browsers);
        assert!(matches!(result, Err(LaunchError::NoDefaultBrowser)));
    }

    #[tokio::test]
    async fn launch_command_spawns_existing_binary() {
        let result = launch_command(&"true".to_string(), None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn launch_command_reports_error_for_missing_binary() {
        let result = launch_command(&"this-binary-does-not-exist".to_string(), None).await;
        assert!(matches!(result, Err(LaunchError::Spawn(_))));
    }

    #[test]
    fn display_messages_are_human_readable() {
        assert_eq!(
            LaunchError::NoDefaultBrowser.to_string(),
            "no default browser configured"
        );

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "binary not found");
        assert!(LaunchError::Spawn(io_err)
            .to_string()
            .starts_with("failed to spawn process"));
    }
}
