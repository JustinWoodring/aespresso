//! Static application metadata and user-facing messages.

pub const APP_ID: &str = "com.justinwoodring.aespresso";
pub const APP_TITLE: &str = "aespresso";
pub const APP_ICON: &str = "aespresso";
/// Kept in sync with `Cargo.toml` at compile time.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Justin Woodring";

pub const REPO_URL: &str = "https://github.com/JustinWoodring/aespresso";
pub const ISSUE_URL: &str = "https://github.com/JustinWoodring/aespresso/issues";

pub const ERR_UNSUPPORTED: &str = "Arch Linux or a supported Arch-based distribution \
(Artix, Manjaro, EndeavourOS) was not detected. \
This tool drives the archlinux-java script and may misbehave on other distributions.";
pub const ERR_NO_SUDO: &str =
    "Could not run a privilege escalation helper. Please install polkit (pkexec) or lxqt-sudo.";
pub const ERR_NO_JAVA: &str =
    "Could not run archlinux-java. Please ensure that it is installed on your system.";
pub const ERR_NO_SELECTION: &str = "Select a Java environment from the list first.";
