mod imp;
mod openai_model;

use gstreamer::{glib, prelude::StaticType, Rank};

glib::wrapper! {
  pub struct OpenaiChatFilter(ObjectSubclass<imp::OpenaiChatFilter>) @extends gstreamer_base::BaseTransform, gstreamer::Element, gstreamer::Object;
}

pub fn register(plugin: &gstreamer::Plugin) -> Result<(), glib::BoolError> {
  gstreamer::Element::register(
    Some(plugin),
    "openaichat",
    Rank::None,
    OpenaiChatFilter::static_type(),
  )
}
