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
    Credentials::Token(String::from(personal_token)),
).expect("client error");

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

#[tauri::command]
async fn fetch_repo_info(personal_token:String) 
  -> Vec<TrafficInfo>  {
  let result = 
    get_reqwest(personal_token).await.expect("func error");

  result.into()
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![fetch_repo_info])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
