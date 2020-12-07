extern crate gio;
extern crate gtk;

use crate::app_constants::AppData;
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Adjustment, Application, ApplicationWindow, Box, Button, ButtonBox, Frame, Label, LinkButton,
    ScrolledWindow,
};

pub fn new(app: &Application, app_data: &AppData) -> ApplicationWindow {
    //Create new window to be returned with information provided by the given AppData.
    let about_window = ApplicationWindow::new(app);
    about_window.set_default_size(400, 400);
    about_window.set_resizable(false);
    about_window.set_title("About");
    let about_window_clone = about_window.clone();
    let about_window_clone2 = about_window.clone();
    let about_window_clone3 = about_window.clone();
    //Do not destroy window only hide it.
    about_window_clone.connect_delete_event(move |_, _| {
        about_window_clone2.hide();
        Inhibit(true)
    });
    let about_box = Box::new(Vertical, 10);
    let details_box = Box::new(Vertical, 10);
    let scroll_box_wrapper = Frame::new(Some("License"));
    let scroll_box = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    scroll_box.set_size_request(300, 400);
    let button_box = ButtonBox::new(Horizontal);
    let label_name = Label::new(None);
    label_name.set_markup(&format!("<b>{}</b>", app_data.title));
    let label_version = Label::new(None);
    label_version.set_markup(&("<b>Version: </b>".to_string() + &app_data.version.to_string()));
    let label_authors = Label::new(None);
    label_authors.set_markup(&("<b>Authors: </b>".to_string() + &app_data.authors.to_string()));
    let label_repo_link = LinkButton::with_label(&app_data.repo.0, Some(&app_data.repo.1));

    let label_license = Label::new(Some(app_data.license));

    label_license.set_line_wrap(true);
    let label_license_wrapper = Box::new(Vertical, 0);
    let close_about = Button::with_label("Close");
    close_about.set_tooltip_text(Some("Close this window."));
    close_about.connect_clicked(move |_| {
        about_window_clone3.hide();
    });

    details_box.add(&label_name);
    details_box.add(&label_version);
    details_box.add(&label_authors);
    details_box.add(&label_repo_link);
    scroll_box.add(&label_license_wrapper);
    scroll_box_wrapper.add(&scroll_box);
    button_box.add(&close_about);
    about_box.add(&details_box);
    about_box.add(&scroll_box_wrapper);
    about_box.add(&button_box);
    about_window.add(&about_box);
    label_license_wrapper.add(&label_license);
    label_license_wrapper.set_border_width(10);
    about_window.set_border_width(10);
    return about_window;
}
