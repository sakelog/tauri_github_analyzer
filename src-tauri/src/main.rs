#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use dotenv::dotenv;
use octorust::{auth::Credentials, Client};
// use serde_json::json;

#[allow(dead_code)]
#[derive(serde::Serialize)]
  struct ViewItem {
      name: String,
      count: i64,
      uniques: i64,
      views: Vec<octorust::types::Traffic>,
  }

async fn get_reqwest() -> Result<Vec<ViewItem>,octorust::types::Error> {
  dotenv().ok();
  let personal_token = 
  dotenv::var("REACT_APP_GITHUB_PERSONAL_ACCESS_TOKEN")
  .expect("ACCESS_TOKEN must be set");

  let github = Client::new(
    String::from("user-agent-name"),
    Credentials::Token(String::from(personal_token)),
).expect("client error");

  let user_resp = 
    github
    .users()
    .get_authenticated_public_user()
    .await.expect("user get error");

  let login = user_resp.login;

  let resp =
    github
    .repos()
    .list_all_for_user(
      &login,
      octorust::types::ReposListUserType::Owner, 
      octorust::types::ReposListOrgSort::Created, 
      octorust::types::Order::Desc)
    .await.expect("repos get error");
    
  let mut views_list :Vec<ViewItem> = vec![];
  
  for repo in &resp {
    let views = 
      github
      .repos()
      .get_views(
        &login,
        &repo.name,
        octorust::types::Per::Day
      )
      .await.expect("get view error");

    let views_item = ViewItem {
      name:repo.name.clone(),
      count: views.count,
      uniques: views.uniques,
      views: views.views,
    };

    views_list.push(views_item);
  }
  // println!("Result: {:?}", views_list);
  Ok(views_list)
}

#[tauri::command]
async fn my_custom_command() -> Vec<ViewItem>  {
  let result = 
    get_reqwest().await.expect("func error");
  //println!("Result: {:?}", result);

  result.into()
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![my_custom_command])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
