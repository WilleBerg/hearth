//! Platform abstraction for launching applications and OS-specific actions
//! (shutdown, reboot). Keeps `app`/`ui` portable across desktop and Pi
//! targets. Populated in a later milestone.
//!

use std::fmt;
use std::process::Command;

use crate::config::browsers::Browsers;
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

pub fn launch_command(command: String, args: Option<Vec<String>>) -> Result<(), LaunchError> {
    let args = args.unwrap_or(vec![]);
    info!("Launching command: {command} {args:#?}");
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(LaunchError::Spawn)?;
    debug!("launch_command output: {output:#?}");
    Ok(())
}

pub fn launch_url(
    url: String,
    browser: Option<String>,
    browsers: &Browsers,
) -> Result<(), LaunchError> {
    let browser = browser
        .as_ref()
        .and_then(|name| browsers.browsers.get(name))
        .or_else(|| browsers.browsers.get(&browsers.default))
        .ok_or(LaunchError::NoDefaultBrowser)?;
    let command = browser.command.clone();
    let args = browser.args.clone();
    info!("Launching url: {command} {args:#?} {url}");
    let output = Command::new(command)
        .args(args)
        .arg(url)
        .output()
        .map_err(LaunchError::Spawn)?;

    debug!("launch_command output: {output:#?}");

    Ok(())
}
