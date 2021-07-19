use std::cell::Cell;
use std::process::Command;

use gtk::{Adjustment, Application, ApplicationWindow, Scrollbar};
use gtk::prelude::*;

static VALUE_CHANGED_SIGNAL: &str = "value-changed";
static ZOOM_MIN: f64 = 100.0;
static ZOOM_MAX: f64 = 500.0;
static ZOOM_CLICK: f64 = 10.0; // Minimum change before we re-issue the svl2-ctl command to change Zoom level

fn main() {
    let state = Cell::new(100.0);

    let application = Application::builder()
        .application_id("com.paperstack.Zoomies")
        .build();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("BRIO Zoomies")
            .default_width(350)
            .default_height(70)
            .build();

        // HBox to lay out the slider and output
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);

        // Slider: Note that the adjustment values aren't honoured by the slider in the way I expected. Need to figure out what I'm doing wrong here.
        let adjustment = Adjustment::new(ZOOM_MIN, ZOOM_MIN, ZOOM_MAX, ZOOM_CLICK, 0.0, 0.0);
        let scrollbar = Scrollbar::new(gtk::Orientation::Horizontal, Option::from(&adjustment));
        hbox.pack_start(&scrollbar, true, true, 0);

        // Display the current zoom level
        let label = gtk::Label::new(Some((ZOOM_MIN as i64).to_string().as_str()));
        hbox.pack_start(&label, false, false, 2);

        let state_copy = state.clone();
        let _ = scrollbar.connect_local(VALUE_CHANGED_SIGNAL, false, move |values| {
            for value in values {
                let scroll = value.get::<Scrollbar>().unwrap();
                let scrolled_to = scroll.value();
                adjusted(scrolled_to, &state_copy);
                label.set_text((state_copy.get() as i64).to_string().as_str());
            }
            None
        });

        window.add(&hbox);
        window.show_all();
    });

    application.run();
}

fn adjusted(value: f64, state: &Cell<f64>) {
    if moved_enough(value, state) {
        state.replace(value);
        zoom(state.get());
    }
}

fn moved_enough(value: f64, state: &Cell<f64>) -> bool {
    if value <= ZOOM_MIN || value >= ZOOM_MAX {
        state.get() != value
    } else {
        (state.get() - value).abs() > ZOOM_CLICK
    }
}

fn zoom(value: f64) {
    // Works-on-my-machine but should configure by reading current devices and available ranges
    // and with better errors if the command's not available and so on.
    Command::new("v4l2-ctl")
        .arg("-d")
        .arg("/dev/video2")
        .arg(format!("--set-ctrl=zoom_absolute={}", value))
        .output()
        .expect("Well oops.");
}