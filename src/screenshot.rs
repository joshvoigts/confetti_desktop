use bevy::prelude::*;
use std::env;
use xcap::Monitor;

#[derive(Resource, Default)]
pub struct Screenshot {
   pub height: f32,
   pub width: f32,
   pub scale: f32,
   pub path: String,
}

impl Screenshot {
   pub fn capture() -> Self {
      // Screenshot background
      let monitors = Monitor::all().unwrap();
      let monitor =
         monitors.iter().find(|&m| m.is_primary()).unwrap();

      let image = monitor.capture_image().unwrap();
      let mut path_buf = env::temp_dir();
      path_buf.push("tmp_screenshot.png");
      let path = path_buf.to_string_lossy().into_owned();
      image.save(path.clone()).unwrap();

      let scale = 1.0 / monitor.scale_factor();
      let width = (monitor.width() as f32) / monitor.scale_factor();
      let height = (monitor.height() as f32) / monitor.scale_factor();

      Self {
         height: height,
         width: width,
         scale: scale,
         path: path,
      }
   }
}
