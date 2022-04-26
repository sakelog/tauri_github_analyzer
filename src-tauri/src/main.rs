#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(Debug)]
#[allow(dead_code)]
#[derive(serde::Serialize)]
  struct TrafficInfo {
      name: String,
      url: String,
      views_info : ViewsInfo,
      clones_info: ClonesInfo,
  }
  #[derive(Debug)]
  #[derive(serde::Serialize)]
  struct ViewsInfo {
    count: i64,
    uniques: i64,
    items: Vec<octorust::types::Traffic>,
  }
  #[derive(Debug)]
  #[derive(serde::Serialize)]
  struct ClonesInfo {
    count: i64,
    uniques: i64,
    items: Vec<octorust::types::Traffic>,
  }


use octorust::{auth::Credentials, Client, types::MinimalRepository};  
use std::env;

async fn get_login_user(github:Client) -> 
  String {
  let resp = 
    github
    .users()
    .get_authenticated_public_user()
    .await.expect("user get error");

  let login = resp.login;

  login.into()
}

async fn get_repos(github:Client,login:String) 
  -> Vec<MinimalRepository> {
  let resp =
    github
    .repos()
    .list_all_for_user(
      &login,
      octorust::types::ReposListUserType::Owner, 
      octorust::types::ReposListOrgSort::Created, 
      octorust::types::Order::Desc)
    .await.expect("repos get error");

  resp.into()
}

async fn get_reqwest(personal_token:String) -> 
  Result<Vec<TrafficInfo>,octorust::types::Error> {

  let github = Client::new(
    String::from("user-agent-name"),
    Credentials::Token(personal_token.clone()),
  )
  .expect("client error");

  let login = 
    get_login_user(github.clone()).await;
    
  let repos = 
    get_repos(github.clone(), login.clone()).await;

  let mut traffic_list :Vec<TrafficInfo> = vec![];
  
  for repo in &repos {
    let views = 
      github
      .repos()
      .get_views(
        &login,
        &repo.name,
        octorust::types::Per::Day
      )
      .await.expect("get view error");

    let clones =
      github
      .repos()
      .get_clones(
        &login,
        &repo.name,
        octorust::types::Per::Day
      ).await.expect("get clones error");

    let traffic_result = TrafficInfo {
      name:repo.name.clone(),
      url: repo.html_url.clone(),
      views_info: { ViewsInfo {
        count: views.count,
        uniques: views.uniques,
        items: views.views,
      } },
      clones_info: { ClonesInfo {
         count: clones.count, 
         uniques: clones.uniques, 
         items: clones.clones, 
        }

      }
    };

    traffic_list.push(traffic_result);
  }
  
  Ok(traffic_list)
}

use tauri::api::path::{app_dir};
use std::fs;
use std::fs::{OpenOptions};
use std::io::{Write};

use std::path::PathBuf;

fn create_env_file(
  dir_path:PathBuf,
  file_path:PathBuf,
  personal_token:String,
) {
  let personal_token_json = serde_json::json!(
    {
      "personal_token" : personal_token.clone(),
    }
  );

 if dir_path.exists() {
 } else {
  fs::create_dir(dir_path)
    .unwrap_or_else(|why| {
      println!("!dir: {:?}", why.kind())});
  }

  let mut output_file = 
    OpenOptions::new()
    .create(true)
    .write(true)
    .open(file_path)
    .expect("file create error");

  let out_str =
    serde_json::to_string(&personal_token_json)
    .expect("json toStr error");
  
  let out_buf = out_str.as_bytes();
  output_file.write_all(out_buf).expect("write file error");
  output_file.flush().expect("flush file error");
}

fn load_env(
  file_path:PathBuf
) -> String {
  let input_str = fs::read_to_string(file_path)
    .expect("file read error");

  let input_json :serde_json::Value = 
    serde_json::from_str(&input_str)
    .expect("str toJson error");

  input_json["personal_token"].as_str().unwrap().into()
}

#[tauri::command]
async fn fetch_repo_info(new_personal_token:Option<String>) 
  -> Vec<TrafficInfo>  {

  let mut path_json_dir = 
    app_dir(&Default::default()).unwrap();
  path_json_dir.push("Tauri Github Analyzer");

  let mut path_json_file = path_json_dir.clone();
  path_json_file.push("token.json");

  if path_json_file.exists() {
  } else {
  match new_personal_token {
    Some(result) => create_env_file(
      path_json_dir,
      path_json_file.clone(),
      result),
    None => panic!("personal_token must set")
    }
  }

  let exist_personal_token = 
    load_env(path_json_file);

  let result = 
    get_reqwest(exist_personal_token).await.expect("func error");

  result.into()
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![fetch_repo_info])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
