//! Helpers for shelling out to `archlinux-java`, refreshing the environment
//! list, and showing message dialogs.

use gtk::gio;
use gtk::glib;
use gtk::glib::object::IsA;
use gtk::prelude::*;
use gtk::{
    ButtonsType, Label, LinkButton, ListBox, ListBoxRow, MessageDialog, MessageType, Window,
};
use std::path::PathBuf;
use std::process::Command;

use crate::app_constants as app;

/// A Java environment as reported by `archlinux-java status`.
pub struct JavaEnv {
    pub name: String,
    pub is_default: bool,
}

/// Name of a Java environment, or the exact text of an error to show.
type JavaStatus = Result<Vec<JavaEnv>, String>;

fn find_in_path(program: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|path| {
        std::env::split_paths(&path)
            .map(|dir| dir.join(program))
            .find(|candidate| candidate.is_file())
    })
}

/// Pick a privilege escalation helper. `pkexec` (polkit) is the standard on
/// modern desktops; `lxqt-sudo` is kept as a fallback for existing setups.
fn sudo_program() -> Option<&'static str> {
    ["pkexec", "lxqt-sudo"].into_iter().find(|p| find_in_path(p).is_some())
}

/// `archlinux-java status`, parsed into environments. The `(default)` marker
/// is split off into [`JavaEnv::is_default`].
fn java_status() -> JavaStatus {
    let output = Command::new("archlinux-java")
        .arg("status")
        .output()
        .map_err(|_| app::ERR_NO_JAVA.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("java-"))
        .map(|line| match line.strip_suffix(" (default)") {
            Some(name) => JavaEnv { name: name.to_string(), is_default: true },
            None => JavaEnv { name: line.to_string(), is_default: false },
        })
        .collect())
}

/// Run `archlinux-java <args>` through a privilege escalation helper.
fn run_privileged(java_args: &[String]) -> Result<(), String> {
    let sudo = sudo_program().ok_or(app::ERR_NO_SUDO)?;
    let mut command = Command::new(sudo);
    if sudo == "lxqt-sudo" {
        command.arg("-s");
    }
    let output = command
        .arg("archlinux-java")
        .args(java_args)
        .output()
        .map_err(|_| app::ERR_NO_SUDO.to_string())?;

    // An empty stderr with a non-zero status (e.g. a cancelled pkexec
    // prompt) is not worth an error dialog; anything else is.
    if !output.status.success() && !output.stderr.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(())
}

/// Show a message dialog. Safe to call from any UI context; blocks nothing.
pub fn show_message<W: IsA<Window>>(
    parent: Option<&W>,
    message_type: MessageType,
    text: &str,
    link: Option<(&str, &str)>,
) {
    let dialog = MessageDialog::builder()
        .message_type(message_type)
        .buttons(ButtonsType::Ok)
        .text(text)
        .modal(true)
        .build();
    if let Some(parent) = parent {
        dialog.set_transient_for(Some(parent));
    }
    if let Some((uri, label)) = link {
        dialog.content_area().append(&LinkButton::with_label(uri, label));
    }
    dialog.connect_response(|dialog, _| dialog.destroy());
    dialog.present();
}

/// The environment name of the current selection, without the `(default)`
/// suffix, ready to pass to `archlinux-java set`.
pub fn selected_env(listbox: &ListBox) -> Option<String> {
    let row = listbox.selected_row()?;
    let label = row.child()?.downcast::<Label>().ok()?;
    Some(label.label().trim_end_matches(" (default)").to_string())
}

/// Run a closure on a background thread and await its result on the main
/// loop, keeping the UI responsive.
async fn on_blocking_thread<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    gio::spawn_blocking(f)
        .await
        .expect("background task panicked")
}

/// Repopulate the list from `archlinux-java status`, running the command on a
/// background thread so the UI never blocks.
pub fn refresh_env<W: IsA<Window>>(window: &W, listbox: &ListBox) {
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }

    let window = window.clone();
    let listbox = listbox.clone();
    glib::spawn_future_local(async move {
        match on_blocking_thread(java_status).await {
            Ok(envs) => {
                for env in envs {
                    let text = if env.is_default {
                        format!("{} (default)", env.name)
                    } else {
                        env.name
                    };
                    let row = ListBoxRow::new();
                    row.set_child(Some(&Label::new(Some(&text))));
                    listbox.append(&row);
                }
            }
            Err(message) => show_message(Some(&window), MessageType::Error, &message, None),
        }
    });
}

/// Run a privileged `archlinux-java` subcommand, then refresh the list. The
/// window is made insensitive while the command runs so actions can't pile up.
pub fn run_java_action<W: IsA<Window> + IsA<gtk::Widget>>(
    window: &W,
    listbox: &ListBox,
    java_args: &[&str],
) {
    window.set_sensitive(false);
    let java_args: Vec<String> = java_args.iter().map(|s| s.to_string()).collect();

    let window = window.clone();
    let listbox = listbox.clone();
    glib::spawn_future_local(async move {
        let result = on_blocking_thread(move || run_privileged(&java_args)).await;
        window.set_sensitive(true);
        if let Err(message) = result {
            show_message(Some(&window), MessageType::Error, &message, None);
        }
        refresh_env(&window, &listbox);
    });
}

/// Distribution identifier from `/etc/os-release`, if readable.
pub fn detect_os_id() -> Option<String> {
    std::fs::read_to_string("/etc/os-release").ok().and_then(|content| {
        content.lines().find_map(|line| {
            line.strip_prefix("ID=").map(|id| id.trim_matches('"').to_string())
        })
    })
}
