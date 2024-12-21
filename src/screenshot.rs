use bevy::prelude::*;
use std::env;
use std::{thread, time};
use xcap::Monitor;

#[cfg(target_os = "macos")]
use crate::macos::{preflight_access, request_access};

#[derive(Resource, Default)]
pub struct Screenshot {
   pub height: f32,
   pub width: f32,
   pub scale: f32,
   pub path: String,
}

#[cfg(target_os = "macos")]
pub fn test_access() -> bool {
   if !preflight_access() {
      request_access();
      return false;
   }
   return true;
}

#[cfg(not(target_os = "macos"))]
pub fn test_access() -> bool {
   return true;
}

impl Screenshot {
   pub fn capture() -> Option<Self> {
      if !test_access() {
         return None;
      }

      // Sleep a bit otherwise we might capture the app
      // starting animation.
      thread::sleep(time::Duration::from_secs(2));

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
      let width = monitor.width() as f32;
      let height = monitor.height() as f32;

      Some(Self {
         height: height,
         width: width,
         scale: scale,
         path: path,
      })
   }
}
