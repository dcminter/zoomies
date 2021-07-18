# Zoomies

Super simple Rust/Gtk GUI to control the Zoom level on my Logitech BRIO camera.

This can be done from the command line (during use of the camera by another app) with something like the following video4linux command:

```bash
v4l2-ctl -d /dev/video2 --set-ctrol=zoom_absolute=150
```

The GUI tool will allow the appropriate zoom level to be selected with a slider (device, range. etc. to be hardcoded at least initially).

## Notes

  * On Ubuntu the video4linux `v4l2-ctl` command comes from the `v4l-utils` package.