#[derive(Debug)]
#[allow(dead_code)]
#[derive(serde::Serialize)]
pub struct TrafficInfo {
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

use anyhow::Result;
use octorust::{auth::Credentials, Client, types::MinimalRepository};  

async fn get_login_user(github:Client) -> 
  Result<String> {
  let resp = 
    github
    .users()
    .get_authenticated_public_user()
    .await;

  let resp = match resp {
    Ok(resp) => resp,
    Err(e) => return Err(e),
  };

  let login = resp.login;

  Ok(login.into())
}

async fn get_repos(github:Client,login:String) 
  -> Result<Vec<MinimalRepository>> {
  let repos =
    github
    .repos()
    .list_all_for_user(
      &login,
      octorust::types::ReposListUserType::Owner, 
      octorust::types::ReposListOrgSort::Created, 
      octorust::types::Order::Desc)
    .await;

  match repos {
    Ok(repos) => return Ok(repos),
    Err(e) => return Err(e)
  }
}

pub async fn get_reqwest(personal_token:String) -> 
  Result<Vec<TrafficInfo>> {

  let github = Client::new(
    String::from("user-agent-name"),
    Credentials::Token(personal_token.clone()),
  )
  .expect("client error");

  let login = 
    get_login_user(github.clone()).await;

  let login = match login {
    Ok(login) => login,
    Err(e) => return Err(e),
  };
    
  let repos = 
    get_repos(github.clone(), login.clone()).await;

  let repos = match repos {
    Ok(repos) => repos,
    Err(e) => return Err(e),
  };

  let mut traffic_list :Vec<TrafficInfo> = vec![];
  
  for repo in &repos {
    let views = 
      github
      .repos()
      .get_views(
        &login.clone(),
        &repo.name,
        octorust::types::Per::Day
      )
      .await.expect("get view error");

    let clones =
      github
      .repos()
      .get_clones(
        &login.clone(),
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
  
  Ok(traffic_list.into())
}