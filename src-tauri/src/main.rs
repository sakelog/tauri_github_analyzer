#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use dotenv::dotenv;
use octorust::{auth::Credentials, Client};

#[allow(dead_code)]
#[derive(serde::Serialize)]
  struct TrafficInfo {
      name: String,
      url: String,
      views_info : ViewsInfo,
      clones_info: ClonesInfo,
  }
  #[derive(serde::Serialize)]
  struct ViewsInfo {
    count: i64,
    uniques: i64,
    items: Vec<octorust::types::Traffic>,
  }
  #[derive(serde::Serialize)]
  struct ClonesInfo {
    count: i64,
    uniques: i64,
    items: Vec<octorust::types::Traffic>,
  }

async fn get_reqwest() -> Result<Vec<TrafficInfo>,octorust::types::Error> {
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
    
  let mut traffic_list :Vec<TrafficInfo> = vec![];
  
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
async fn my_custom_command() -> Vec<TrafficInfo>  {
  let result = 
    get_reqwest().await.expect("func error");

  result.into()
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![my_custom_command])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
