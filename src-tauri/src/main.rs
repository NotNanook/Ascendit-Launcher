
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{io::{Cursor, Read}, fs::File, path::Path};
use sha256::{digest_file};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
extern crate directories;
use directories::{ProjectDirs};

fn main() 
{
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![inject_raot, save_changes])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn check_update(url: String, game: &str) -> Result<()> {
  if let Some(proj_dirs) = ProjectDirs::from("", "",  "Ascendit Launcher") {
    // get path to folder of gamedll
    let proj_path= proj_dirs.data_dir();
    let prefix = proj_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    // download most recent hash from server
    let game_hash_str: &str = &("hash".to_owned() + game + &(".txt".to_owned()));
    let hash_new = reqwest::get(url.clone() + game_hash_str).await?.text().await?;

    // hash file when already existing
    let dll_path_buf = proj_path.join("Ascendit-".to_owned() + game + &(".dll".to_owned()));
    let dll_path = dll_path_buf.as_path();
    if dll_path.exists() {
      let hash = digest_file(dll_path).unwrap();

      if hash_new.trim_end().ne(&hash) {
        let response = reqwest::get(url + "Ascendit-" + game + ".dll").await?;
        let mut file = std::fs::File::create(dll_path)?;
        let mut content =  Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
      }
    }

    Ok(())
  } else {
    Ok(())
  }
}

#[tauri::command]
async fn inject_raot()
{
  // check for updates
  let _check_result = check_update("https://v45600.1blu.de/launcherFiles/".to_string(), "Raot").await.unwrap();
  
  // execute
  // inject
}

#[tauri::command]
fn save_changes() 
{
  
}