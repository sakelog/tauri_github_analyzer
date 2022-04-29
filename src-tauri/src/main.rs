#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use tauri::api::path::app_dir;
use tauri_github_analyzer::{get_reqwest,load_env,create_env_file};

struct PathState(PathBuf);

fn get_target_path() -> PathBuf {
  const APP_NAME : &str = "Tauri GitHub Analyzer";
  const FILE_NAME : &str = "token.json";

  let mut path_json_file = 
    app_dir(&Default::default()).unwrap();
  path_json_file.push(APP_NAME);
  path_json_file.push(FILE_NAME);

  path_json_file.into()
}

#[tauri::command]
fn check_exist_file(state: tauri::State<'_, PathState>) -> bool {
  let target_file_path = &state.0;
  let result =
    if target_file_path.exists() {
      true
    } else {
      false
    };

  result.into()
}

#[tauri::command]
async fn fetch_repo_info(
  new_personal_token:Option<String>,
  state: tauri::State<'_, PathState>
) 
  -> Result<String,String>  {

match new_personal_token {
  Some(token) => create_env_file(
    state.0.clone(),
    token),
  None => ()
  };

  let exist_personal_token = 
    load_env(state.0.clone());

  let fetch_result = 
    get_reqwest::main(exist_personal_token).await;

  let result_json = match fetch_result {
    Ok(fetch_result) => 
      serde_json::json!(fetch_result)
      .to_string(),
    Err(_) => return Err("fetch repo info error".to_string()),
  };

  Ok(result_json)
}

#[tauri::command]
fn delete_file(state: tauri::State<PathState>) {
  let file_path = &state.0;
    if file_path.exists() {
    fs::remove_file(file_path)
    .expect("file delete error")
  }
}

#[tauri::command]
fn abnormal_end() -> Result<(),()> {
  panic!("token send limit over")
}

use std::env;

fn main() {
  tauri::Builder::default()
  .manage(PathState(get_target_path().into()))
  .invoke_handler(tauri::generate_handler![
    check_exist_file, 
    fetch_repo_info,
    delete_file,
    abnormal_end
  ])
  .run(tauri::generate_context!("tauri.conf.json"))
  .expect("error while running tauri application");
}
