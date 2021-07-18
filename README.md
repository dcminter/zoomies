# Zoomies

Super simple Rust/Gtk GUI to control the Zoom level on my Logitech BRIO camera.

This can be done from the command line (during use of the camera by another app) with something like the following video4linux command:

```bash
v4l2-ctl -d /dev/video2 --set-ctrl=zoom_absolute=150
```

The GUI tool will allow the appropriate zoom level to be selected with a slider (device, range. etc. to be hardcoded at least initially).

## Notes

  * On Ubuntu the video4linux `v4l2-ctl` command comes from the `v4l-utils` package.

## TODO

  * See if there's a simpler way to pass around the state to make it available in the closure.
  * Add visual feedback of the zoom level
  * Fix wonkiness on the slider control range
  * Better handling of the device(s) and ranges for the camera
  * Better error handling for the v4l command (e.g. what if it's not installed!)
  * Display the current zoom level
  * Use v4l2 API more directly so there's no need to invoke the v4l2-ctl binary (see [v4l2-ctl implementation](https://github.com/gjasny/v4l-utils/tree/master/utils/v4l2-ctl) and e.g. [rust v4l crate](https://docs.rs/v4l/0.12.1/v4l/))
