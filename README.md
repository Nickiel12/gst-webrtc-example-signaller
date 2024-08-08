A compilable clone of the [official webrtc signaller example](https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/net/webrtc/examples/webrtcsink-custom-signaller/main.rs).

The main reason behind publishing this repo is that I could only find one example of this code in the wild, and it took me a long time to get the right imports, so I made this example.

To run this code clone the repository, and make sure gstreamer, gst-plugins base, and bad are installed. **Do not install gst-plugins-rs** as when this library is loaded, it clashes with the gstrswebrtc library that registers the required plugins and elements at program time.


## Original README from source

# WebRTCSink custom signaller
A simple application that consist of two parts:

main executable, which demonstrates how to instantiate WebRTCSink with a custom signaller

signaller module, which provides all the required boilerplate code
and stub implementations needed to create a custom signaller

Run with:
```sh
cargo run --example webrtcsink-custom-signaller
```
The expected output is a not-implemented panic (from imp::Signaller::start function):
```sh
thread 'tokio-runtime-worker' panicked at 'not implemented', net/webrtc/examples/webrtcsink-custom-signaller/signaller/imp.rs:14:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
Simply implement the methods in imp.rs and you should be good
to go!