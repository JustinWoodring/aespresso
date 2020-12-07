use gio::prelude::*;
use gtk::prelude::*;
use gtk::ResponseType;
use gtk::{Application, Dialog, DialogFlags, Label, LinkButton, ListBox, ListBoxRow, Window};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

pub fn refresh_env(
    app: &Application,
    err_msg: crate::app_constants::ErrMsg,
    listbox: &ListBox,
    list: &Rc<RefCell<Vec<ListBoxRow>>>,
) {
    //Loop through object list and remove objects from the ListBox.
    for x in 0..list.borrow().len() {
        listbox.remove(&list.borrow()[x]);
    }

    //Empty our list.
    list.borrow_mut().clear();

    //Build command to get data from the console.
    let list_fetch = Command::new("archlinux-java").arg("status").output();

    let mut output = String::new();

    match list_fetch {
        Ok(result) => {
            //Remove unwanted text parts from output.
            output = String::from_utf8(result.stdout)
                .unwrap()
                .replace("Available Java environments:\n", "")
                .replace("  ", "")
                .replace("No Java environment set as default", "")
                .replace("No compatible Java environment installed", "");
        }
        Err(_text) => {
            show_message(&app, Err(err_msg.get_err(2)), None);
        }
    }

    /*Split output on the \n to a list of options. Loop through
    and create corresponding ListBoxRows with child elements of Labels
    containing the option and append these to the list.*/
    for x in output.split("\n") {
        let label = Label::new(Some(x));
        let element = ListBoxRow::new();
        element.add(&label);
        list.borrow_mut().push(element);
    }
    //Remove one element to account for the last \n.
    list.borrow_mut().pop();

    /*add let no_env = Label::new(Some("No Java Environments Found"));*/

    //Loop through the list and add the object into the ListBox.
    for x in 0..list.borrow().len() {
        listbox.add(&list.borrow()[x]);
        list.borrow()[x].show();
        list.borrow()[x]
            .get_child()
            .unwrap()
            .downcast::<Label>()
            .unwrap()
            .show();
    }
}

pub fn show_message(
    app: &Application,
    message: Result<&str, &str>,
    uri: Option<(&str, Option<&str>)>,
) {
    match message {
        Ok(text) => {
            let dialog = Dialog::with_buttons(
                Some("Info"),
                None::<&Window>,
                DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok)],
            );
            dialog.set_default_size(400, 100);
            dialog.set_resizable(false);
            let label = Label::new(None);
            label.set_markup(&("<b>Info: </b>".to_string() + &text.to_string()));
            label.set_line_wrap(true);
            dialog.get_content_area().add(&label);
            if let Some((uri, name)) = uri {
                if let Some(text) = name {
                    let link = LinkButton::with_label(uri, Some(text));
                    dialog.get_content_area().add(&link);
                } else {
                    let link = LinkButton::new(uri);
                    dialog.get_content_area().add(&link);
                }
            }
            dialog.set_border_width(10);
            let app_quit2 = app.clone();
            dialog.show_all();
            let response = dialog.run();
            match response {
                ResponseType::Ok => {
                    dialog.close();
                }
                _ => {
                    dialog.close();
                    app_quit2.quit();
                }
            }
        }
        Err(text) => {
            let dialog = Dialog::with_buttons(
                Some("Error"),
                None::<&Window>,
                DialogFlags::MODAL,
                &[("Ok", ResponseType::Close)],
            );
            dialog.set_default_size(400, 100);
            dialog.set_resizable(false);
            let label = Label::new(None);
            label.set_markup(&("<b>Error: </b>".to_string() + &text.to_string()));
            label.set_line_wrap(true);
            dialog.get_content_area().add(&label);
            if let Some((uri, name)) = uri {
                if let Some(text) = name {
                    let link = LinkButton::with_label(uri, Some(text));
                    dialog.get_content_area().add(&link);
                } else {
                    let link = LinkButton::new(uri);
                    dialog.get_content_area().add(&link);
                }
            }
            dialog.set_border_width(10);
            let dialog_clone = dialog.clone();
            let app_quit = app.clone();
            let app_quit2 = app.clone();
            let app_quit3 = app.clone();
            dialog_clone.connect_delete_event(move |_, _| {
                app_quit.quit();
                Inhibit(false)
            });
            dialog.show_all();
            let response = dialog.run();
            match response {
                ResponseType::Close => {
                    dialog.close();
                    app_quit2.quit();
                }
                _ => {
                    dialog.close();
                    app_quit3.quit();
                }
            }
        }
    }
}
