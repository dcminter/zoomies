use std::cell::Cell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{DirEntry, read_to_string};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use gtk::{Adjustment, Application, ApplicationWindow, Scrollbar};
use gtk::prelude::*;
use v4l::control::Description;
use v4l::prelude::*;

static GTK_VALUE_CHANGED_SIGNAL: &str = "value-changed";
static V4L_SYS_DEVICE_PATH: &str = "/sys/class/video4linux";
static BRIO_ZOOM_CONTROL_NAME: &str = "Zoom, Absolute";
static ZOOM_CLICK: f64 = 10.0; // Minimum change before we re-issue the svl2-ctl command to change Zoom level

fn main() {
    match establish_range_and_current_value() {
        Ok((zoom_current, zoom_min, zoom_max)) => {
            eprintln!("Current value: {:?}", zoom_current);

            let state = Cell::new(zoom_current);

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
                let adjustment = Adjustment::new(zoom_min, zoom_min, zoom_max, ZOOM_CLICK, 0.0, 0.0);
                let scrollbar = Scrollbar::new(gtk::Orientation::Horizontal, Option::from(&adjustment));
                scrollbar.set_value(zoom_current);
                hbox.pack_start(&scrollbar, true, true, 0);

                // Display the current zoom level
                let label = gtk::Label::new(Some((zoom_current as i64).to_string().as_str()));
                hbox.pack_start(&label, false, false, 2);

                let state_copy = state.clone();
                let _ = scrollbar.connect_local(GTK_VALUE_CHANGED_SIGNAL, false, move |values| {
                    for value in values {
                        match value.get::<Scrollbar>() {
                            Ok(scroll) => {
                                let scrolled_to = scroll.value();
                                adjusted(zoom_min, zoom_max, scrolled_to, &state_copy);
                                label.set_text((state_copy.get() as i64).to_string().as_str());
                            }
                            Err(e) => {
                                eprintln!("Failed to acquire scrollbar value with error {:?}", e);
                            }
                        }
                    }
                    None
                });

                window.add(&hbox);
                window.show_all();
            });

            application.run();
        }
        Err(e) => {
            eprintln!("Could not find or open camera device. Error: '{:?}", e);
        }
    }
}

fn adjusted(zoom_min: f64, zoom_max: f64, value: f64, state: &Cell<f64>) {
    if moved_enough(zoom_min, zoom_max, value, state) {
        state.replace(value);
        let _ = zoom(state.get() as i32);
    }
}

fn moved_enough(zoom_min: f64, zoom_max: f64, value: f64, state: &Cell<f64>) -> bool {
    if value <= zoom_min || value >= zoom_max {
        state.get() != value
    } else {
        (state.get() - value).abs() > ZOOM_CLICK
    }
}

fn zoom(value: i32) -> Result<(), Error> {
    let dev = brio_device()?;
    let controls = dev.query_controls()?;
    let identifiers: Vec<u32> = controls.into_iter()
        .filter(|c| c.name == BRIO_ZOOM_CONTROL_NAME)
        .map(|c| c.id)
        .collect();

    match identifiers[..] {
        [id] => {
            // Note - set_control doesn't currently accept any control type except Value!
            let _ = dev.set_control(id, v4l::Control::Value(value));
            Ok(())
        }
        _ => {
            eprintln!("No identifier found for the zoom control on the device");
            Err(Error::from(ErrorKind::NotFound))
        }
    }
}

fn brio_device() -> Result<Device, Error> {
    // Find the device. I suspect the "proper" way to do this is to look
    // in /sys/class/video4linux/*/uevent where DEVNAME is specified, but
    // I expect this will mostly be ok using the path name:
    let paths = std::fs::read_dir(Path::new(V4L_SYS_DEVICE_PATH))?;
    let brio_devices: Result<HashMap<i32, OsString>, Error> = paths.into_iter()
        .filter_map(|p| p.ok())
        .filter(correct_device_name)
        .map(to_device_entry)
        .collect();

    // Everything gathered should be a "Logitech BRIO" device
    // TODO: Allow for multiple devices (will require GUI work to support multi-cameras)
    match brio_devices?.get(&0) {
        Some(device) => {
            let mut device_path = PathBuf::new();
            device_path.push("/dev");
            device_path.push(device);

            Device::with_path(device_path)
        }
        None => {
            Err(Error::from(ErrorKind::NotFound))
        }
    }
}

fn to_device_entry(value: DirEntry) -> Result<(i32, OsString), Error> {
    let device: OsString = value.file_name();
    let index: i32 = read_device_index(&value)?;
    Ok((index, device))
}

fn read_device_index(entry: &DirEntry) -> Result<i32, Error> {
    let mut path = entry.path();
    path.push("index");
    let output = read_to_string(path)?.trim().to_string();
    let parsed_output = output.parse::<i32>();
    match parsed_output {
        Ok(value) => {
            Ok(value)
        }
        Err(_) => {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}

fn correct_device_name(entry: &DirEntry) -> bool {
    let mut path = entry.path();
    path.push("name");
    let output = read_to_string(path).unwrap().trim().to_string();
    output == "Logitech BRIO"
}

fn establish_range_and_current_value() -> Result<(f64, f64, f64), Error> {
    let dev = brio_device()?;
    let controls = dev.query_controls()?;

    let zoom_controls: Vec<Description> = controls.into_iter()
        .filter(|c| c.name == BRIO_ZOOM_CONTROL_NAME)
        .collect();

    match &zoom_controls[..] {
        [control_description] => {
            eprintln!("Control: {:?} has range {:?} to {:?}", control_description.name, control_description.minimum, control_description.maximum);

            let current = match dev.control(control_description.id)? {
                v4l::Control::Value(value) => {
                    value
                }
                _ => {
                    control_description.minimum
                }
            };

            Ok((current as f64, control_description.minimum as f64, control_description.maximum as f64))
        }
        _ => {
            Err(Error::from(ErrorKind::NotFound))
        }
    }
}