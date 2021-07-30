# Zoomies

Super simple Rust/Gtk GUI to control the Zoom level on my Logitech BRIO camera.

This can be done from the command line (during use of the camera by another app) with something like the following video4linux command:

```bash
v4l2-ctl -d /dev/video2 --set-ctrl=zoom_absolute=150
```

The GUI tool will allow the appropriate zoom level to be selected with a scrollbar (range. etc. to be hardcoded at least initially).

Clang must be installed (on Ubuntu, `apt install clang`) so that the Rust V4L library dependency can compile. Failure to do so will result in the following error message:
```
error: failed to run custom build command for `v4l-sys v0.2.0`
```

## Notes

  * On Ubuntu the video4linux `v4l2-ctl` command comes from the `v4l-utils` package.

## TODO

  * ~~Read current zoom level so we can initialise the slider to that!~~
  * Do error handling properly
  * ~~See if there's a simpler way to pass around the state to make it available in the closure.~~ I *think* passing the Cell around is a sane way to have the value be both mutable and available to the closure.
  * ~~Add visual feedback of the zoom level~~
  * ~~Fix wonkiness on the slider control range~~ Much less wonky now! Using `connect_local` seems to be the trick! 
  * ~~Better handling of the device(s) and ranges for the camera~~ (Uses v4l directly now)
  * ~~Better error handling for the v4l command (e.g. what if it's not installed!)~~ N/A
  * ~~Display the current zoom level~~ (Added)
  * ~~Use v4l2 API more directly so there's no need to invoke the v4l2-ctl binary (see [v4l2-ctl implementation](https://github.com/gjasny/v4l-utils/tree/master/utils/v4l2-ctl) and e.g. [rust v4l crate](https://docs.rs/v4l/0.12.1/v4l/))~~ In progress on this branch!
