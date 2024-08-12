use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
   pub first_run: bool,
}

impl Settings {
   fn get_path() -> Result<PathBuf> {
      let project =
         ProjectDirs::from("com", "joshvoigts", "confetti_desktop")
            .context("failed to get project directory")?;
      let data_path = project.data_local_dir();
      if !data_path.exists() {
         fs::create_dir(data_path)?;
      }
      Ok(data_path.join("settings.json"))
   }

   pub fn load() -> Result<Self> {
      let path = Self::get_path()?;
      if path.exists() {
         let file = fs::File::open(path)?;
         Ok(serde_json::from_reader(file)?)
      } else {
         let settings = Self { first_run: true };
         settings.save()?;
         Ok(settings)
      }
   }

   pub fn save(&self) -> Result<()> {
      let path = Self::get_path()?;
      let file = fs::File::create(path)?;
      Ok(serde_json::to_writer_pretty(file, &self)?)
   }
}
