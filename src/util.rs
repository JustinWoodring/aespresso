use gtk::ResponseType;
use std::process::Command;
use std::cell::RefCell;
use std::rc::Rc;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ListBox, ListBoxRow, Label, Dialog, Application, Window, DialogFlags, LinkButton};

pub fn refresh_env(listbox : &ListBox, list: &Rc<RefCell<Vec<ListBoxRow>>>){
	//Loop through object list and remove objects from the ListBox.
	for x in 0..list.borrow().len(){
		listbox.remove(&list.borrow()[x]);
	}
	
	//Empty our list.
	list.borrow_mut().clear();
	
	//Build command to get data from the console.
	let list_fetch = Command::new("sh")
     	    .arg("archlinux-java")
            .arg("status")
            .output()
            .expect("failed to execute process");

	//Remove unwanted text parts from output.
	let output = String::from_utf8(list_fetch.stdout).unwrap().replace("Available Java environments:\n", "").replace("  ", "").replace("No Java environment set as default","");
	
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
	for x in 0..list.borrow().len(){
		listbox.add(&list.borrow()[x]);
		list.borrow()[x].show();
		list.borrow()[x].get_child().unwrap().downcast::<Label>().unwrap().show();
	}
}

pub fn show_message(app : &Application, message : Result<&str, &str>, uri : Option<(&str,Option<&str>)>){
	match message {
		Ok(text) => {
			let dialog = Dialog::new_with_buttons(Some("Info"),None::<&Window>,DialogFlags::MODAL,&[("Ok",ResponseType::Ok)]);
			dialog.set_default_size(400, 100);
			dialog.set_resizable(false);
			let label = Label::new(None);
			label.set_markup(&("<b>Info: </b>".to_string() + &text.to_string()));
			label.set_line_wrap(true);
			dialog.get_content_area().add(&label);
			if let Some((uri,name)) = uri{
				if let Some(text) = name {
					let link = LinkButton::new_with_label(uri, Some(text));
					dialog.get_content_area().add(&link);
				}else{
					let link = LinkButton::new(uri);
					dialog.get_content_area().add(&link);
				}
			}
			dialog.set_border_width(10);
			let app_quit2 = app.clone();
			dialog.show_all();
			let response = dialog.run();
			match response {
				ResponseType::Ok =>{
					dialog.destroy();
				}
				_ => {
					dialog.destroy();
					app_quit2.quit();
				}
			}
		}
		Err(text) => {
			let dialog = Dialog::new_with_buttons(Some("Error"),None::<&Window>,DialogFlags::MODAL,&[("Ok",ResponseType::Close)]);
			dialog.set_default_size(400, 100);
			dialog.set_resizable(false);
			let label = Label::new(None);
			label.set_markup(&("<b>Error: </b>".to_string() + &text.to_string()));
			label.set_line_wrap(true);
			dialog.get_content_area().add(&label);
			if let Some((uri,name)) = uri{
				if let Some(text) = name {
					let link = LinkButton::new_with_label(uri, Some(text));
					dialog.get_content_area().add(&link);
				}else{
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
				ResponseType::Close =>{
					dialog.destroy();
					app_quit2.quit();
						
				}
				_ => {
					dialog.destroy();
					app_quit3.quit();
				}
			}
		}
	}
}