use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Orientation, ScrolledWindow, TextView};
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = Application::builder()
        .application_id("com.example.text_file_reader_writer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Text File Reader/Writer")
        .default_width(600)
        .default_height(400)
        .build();

    let vbox = Box::new(Orientation::Vertical, 5);

    let entry = Entry::builder()
        .placeholder_text("Enter file path here")
        .build();

    let text_view = TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);

    let scrolled_window = ScrolledWindow::builder()
        .child(&text_view)
        .vexpand(true)
        .build();

    let read_button = Button::with_label("Read File");
    let write_button = Button::with_label("Write File");

    let entry_ref = Rc::new(entry);
    let text_view_ref = Rc::new(text_view);

    let read_entry_ref = Rc::clone(&entry_ref);
    let read_text_view_ref = Rc::clone(&text_view_ref);
    read_button.connect_clicked(move |_| {
        let file_path = read_entry_ref.text().to_string();
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Failed to open file: {}", file_path);
                return;
            },
        };
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            read_text_view_ref.buffer().set_text(&contents);
        }
    });

    let write_entry_ref = Rc::clone(&entry_ref);
    let write_text_view_ref = Rc::clone(&text_view_ref);
    write_button.connect_clicked(move |_| {
        let file_path = write_entry_ref.text().to_string();
        let buffer = write_text_view_ref.buffer();
        let start_iter = buffer.start_iter();
        let end_iter = buffer.end_iter();
        let text = buffer.text(&start_iter, &end_iter, false).to_string();

        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Failed to create file: {}", file_path);
                return;
            },
        };

        if file.write_all(text.as_bytes()).is_ok() {
            println!("File written successfully: {}", file_path);
        }
    });

    vbox.append(&*entry_ref);
    vbox.append(&scrolled_window);
    vbox.append(&read_button);
    vbox.append(&write_button);

    window.set_child(Some(&vbox));
    window.present();
}
