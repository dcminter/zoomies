use std::cell::Cell;
use std::rc::Rc;

use gtk::{Adjustment, Application, ApplicationWindow, Scrollbar};
use gtk::prelude::*;

use std::process::Command;

static ZOOM_MIN: f64 = 100.0;
static ZOOM_MAX: f64 = 500.0;
static ZOOM_CLICK: f64 = 10.0;

fn main() {

    let state = Rc::new(Cell::new(100.0));

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

        // Add the slider!
        let adjustment = Adjustment::new(ZOOM_MIN, ZOOM_MIN, ZOOM_MAX, 1.0, 1.0, 50.0);
        let scrollbar = Scrollbar::new(gtk::Orientation::Horizontal, Option::from(&adjustment));

        let state_copy = state.clone();
        scrollbar.connect_change_value(move |_, _, value| {
            adjusted(value, &state_copy)
        });
        window.add(&scrollbar);

        window.show_all();
    });

    application.run();
}

fn adjusted(value: f64, state: &Rc<Cell<f64>>) -> Inhibit {
    if f64::abs(state.get() - value) > ZOOM_CLICK {
        if value >= ZOOM_MIN && value <= ZOOM_MAX {
            state.replace(value);
            zoom(state.get());
        }
    }

    Inhibit(false)
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