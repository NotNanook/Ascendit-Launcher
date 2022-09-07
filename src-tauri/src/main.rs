
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{io::{Cursor}, path::Path};
use sha256::{digest_file};
use serde_derive::{Deserialize, Serialize};
use std::process::Command;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
extern crate directories;
use directories::{ProjectDirs};

#[derive(Deserialize, Serialize, Debug)]
struct RaotConfig {
  path: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct MinecraftConfig {
  path: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
  raot: RaotConfig,
  minecraft: MinecraftConfig,
}

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
    std::fs::create_dir_all(proj_path).unwrap();

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
    } else {
      let response = reqwest::get(url + "Ascendit-" + game + ".dll").await?;
      let mut file = std::fs::File::create(dll_path)?;
      let mut content =  Cursor::new(response.bytes().await?);
      std::io::copy(&mut content, &mut file)?;
    }

    Ok(())
  } else {
    Ok(())
  }
}

fn create_config(path: &Path) {
  let raot_path: String = "C:/Users/".to_owned() + whoami::username().as_str() + "/AppData/Local/RAoT/raot.exe";
  let minecraft_path: String = String::from("");

  let mut config_json: ConfigFile = serde_json::from_str(r#"{"raot": {"path": ""},"minecraft" : {"path": ""}}"#).unwrap();
  config_json.raot.path = raot_path;
  config_json.minecraft.path = minecraft_path;

  let _ = std::fs::write(path, serde_json::to_string_pretty(&config_json).unwrap());
}

fn find_path(game: String) -> String {
  if let Some(proj_dirs) = ProjectDirs::from("", "",  "Ascendit Launcher") {
    let proj_path= proj_dirs.data_dir();
    let config_file: &str = "ascendit-config.json";
    let config_path_buf = proj_path.join(config_file);
    let config_path = config_path_buf.as_path();

    match config_path.exists() {
      true => (),
      false => create_config(config_path),
    }

    let config_json = {
      let config_json = std::fs::read_to_string(&config_path).unwrap();
      serde_json::from_str::<ConfigFile>(&config_json).unwrap()
    };
    
    if game == "raot".to_string() {
      return config_json.raot.path;
    } else if game == "minecraft".to_string() {
      return config_json.minecraft.path;
    } else {
      return "".to_owned();
    }

  } else {
    return "".to_owned();
  }
}

fn execute(path: String) {
  Command::new(path).output().expect("Failed to execute process");
}

#[tauri::command]
async fn inject_raot()
{
  // check for updates
  check_update("https://v45600.1blu.de/launcherFiles/".to_string(), "Raot").await.unwrap();
  
  // find path and execute
  let path: String = find_path("raot".to_owned());
  execute(path);
  
  // inject
}

#[tauri::command]
fn save_changes() 
{
  
}