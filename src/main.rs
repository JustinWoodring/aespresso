//! aespresso — a GTK4 frontend for Arch Linux's `archlinux-java` script.

// MessageDialog is deprecated in GTK 4.10 in favor of AlertDialog, but
// AlertDialog cannot host a LinkButton, which we need for issue reporting.
#![allow(deprecated)]

mod app_constants;
mod util;

use app_constants::{
    APP_AUTHOR, APP_ICON, APP_ID, APP_TITLE, APP_VERSION, ERR_NO_SELECTION, ERR_UNSUPPORTED,
    ISSUE_URL, REPO_URL,
};
use gtk::glib::object::IsA;
use gtk::glib::ExitCode;
use gtk::prelude::*;
use gtk::{
    AboutDialog, Application, ApplicationWindow, Box, Button, Frame, License, ListBox,
    MessageType, Orientation, PolicyType, ScrolledWindow, Window,
};

fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .default_width(320)
        .default_height(540)
        .resizable(false)
        .build();

    let root = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Java environment list, scrollable with a comfortable minimum height.
    let env_list = ListBox::new();
    env_list.set_size_request(0, 200);
    let env_scroller = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&env_list)
        .build();
    let env_frame = Frame::builder().label("Java Environments").child(&env_scroller).build();

    let set_default = Button::with_label("Set Default");
    set_default.set_tooltip_text(Some(
        "Sets the selected Java environment as the default environment.",
    ));
    {
        let env_list = env_list.clone();
        let window = window.clone();
        set_default.connect_clicked(move |_| match util::selected_env(&env_list) {
            Some(env) => util::run_java_action(&window, &env_list, &["set", &env]),
            None => util::show_message(
                Some(&window),
                MessageType::Info,
                ERR_NO_SELECTION,
                None,
            ),
        });
    }

    let options_box = Box::builder().orientation(Orientation::Vertical).spacing(8).build();

    let refresh = Button::with_label("Refresh List");
    refresh.set_tooltip_text(Some("Refreshes the Java environment list."));
    {
        let env_list = env_list.clone();
        let window = window.clone();
        refresh.connect_clicked(move |_| util::refresh_env(&window, &env_list));
    }

    let unset = Button::with_label("Unset Default Env");
    unset.set_tooltip_text(Some("Unsets the default Java environment."));
    {
        let env_list = env_list.clone();
        let window = window.clone();
        unset.connect_clicked(move |_| util::run_java_action(&window, &env_list, &["unset"]));
    }

    let fix = Button::with_label("Fix (Auto-select Env)");
    fix.set_tooltip_text(Some(
        "Tries to fix invalid Java environment links. If no default Java environment is set, it will auto-select an environment for you.",
    ));
    {
        let env_list = env_list.clone();
        let window = window.clone();
        fix.connect_clicked(move |_| util::run_java_action(&window, &env_list, &["fix"]));
    }

    let about = Button::with_label("About aespresso");
    about.set_tooltip_text(Some("About this program."));
    {
        let window = window.clone();
        about.connect_clicked(move |_| show_about(&window));
    }

    let close = Button::with_label("Close");
    close.set_tooltip_text(Some("Close this program."));
    {
        let app = app.clone();
        close.connect_clicked(move |_| app.quit());
    }

    for button in [&refresh, &unset, &fix, &about, &close] {
        options_box.append(button);
    }
    let options_frame = Frame::builder().label("Options").child(&options_box).build();

    root.append(&env_frame);
    root.append(&set_default);
    root.append(&options_frame);
    window.set_child(Some(&root));

    match util::detect_os_id().as_deref() {
        Some("arch") | Some("manjaro") => {}
        _ => util::show_message(
            Some(&window),
            MessageType::Warning,
            ERR_UNSUPPORTED,
            Some((ISSUE_URL, "Report an issue?")),
        ),
    }

    util::refresh_env(&window, &env_list);
    window.present();
}

fn show_about<W: IsA<Window>>(parent: &W) {
    let about = AboutDialog::builder()
        .program_name(APP_TITLE)
        .version(APP_VERSION)
        .comments("A GTK4 frontend for Arch Linux's archlinux-java script.")
        .website(REPO_URL)
        .website_label("Git Repository")
        .license_type(License::Bsd)
        .logo_icon_name(APP_ICON)
        .modal(true)
        .transient_for(parent)
        .build();
    about.set_authors(&[APP_AUTHOR]);
    about.present();
}
