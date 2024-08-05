mod signaller;

// from outside the plugin repository, one would need to add plugin package as follows:
// [dependencies]
// gstrswebrtc = { package = "gst-plugin-webrtc", git = "https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/" }
use gstrswebrtc;

use anyhow::Error;
use gstreamer::prelude::*;
use gstrswebrtc::signaller as signaller_interface;
use gstrswebrtc::webrtcsink;

fn main() -> Result<(), Error> {
    gstreamer::init()?;

    let custom_signaller = signaller::MyCustomSignaller::new();
    let webrtcsink = webrtcsink::BaseWebRTCSink::with_signaller(
        signaller_interface::Signallable::from(custom_signaller),
    );

    let pipeline = gstreamer::Pipeline::new();

    let video_src = gstreamer::ElementFactory::make("videotestsrc").build().unwrap();

    pipeline
        .add_many([&video_src, webrtcsink.upcast_ref()])
        .unwrap();
    video_src
        .link(webrtcsink.upcast_ref::<gstreamer::Element>())
        .unwrap();

    let bus = pipeline.bus().unwrap();

    pipeline.set_state(gstreamer::State::Playing).unwrap();

    let _msg = bus.timed_pop_filtered(gstreamer::ClockTime::NONE, &[gstreamer::MessageType::Eos]);

    pipeline.set_state(gstreamer::State::Null).unwrap();

    Ok(())
}
