# Zoomies

Super simple Rust/Gtk GUI to control the Zoom level on my Logitech BRIO camera.

![Zoomies in use](docs/gui.png?raw=true "The (ugly) BRIO Zoomies user interface in action")

This can be done from the command line (during use of the camera by another app) with something like the following video4linux command:

```bash
v4l2-ctl -d /dev/video2 --set-ctrl=zoom_absolute=150
```

The GUI tool will allow the appropriate zoom level to be selected with a scrollbar.

Clang must be installed (on Ubuntu, `apt install clang`) so that the Rust V4L library dependency can compile. Failure to do so will result in the following error message:
```
error: failed to run custom build command for `v4l-sys v0.2.0`
```

## Notes

  * On Ubuntu the video4linux `v4l2-ctl` command comes from the `v4l-utils` package.

## TODO

  * Cut a release
  * Make error handling nicer - all errors are treated as std::io::Error at the moment!
  * Make it prettier (how does Gtk's CSS stuff work‽)
  * Allow for keyboard editing of the zoom level
  * Allow for multiple cameras
