use std::cell::Cell;
use std::rc::Rc;

use gtk::{Adjustment, Application, ApplicationWindow, Scrollbar};
use gtk::prelude::*;

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
        let adjustment = Adjustment::new(100.0, 100.0, 500.0, 1.0, 1.0, 50.0);
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
    if f64::abs(state.get() - value) > 20.0 {
        if value >= 100.0 && value <= 500.0 {
            state.replace(value);
            zoom(state.get());
        }
    }

    Inhibit(false)
}

fn zoom(value: f64) {
    eprintln!("Zoom {:?}", value);
}