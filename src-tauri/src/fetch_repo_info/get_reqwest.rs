use anyhow::{Result,anyhow};
use reqwest::{Client,header};
use serde_json::Value;
use futures::future;
use super::struct_type::{
  TrafficInfo,Repository,ViewsItem,ClonesItem,TrafficItem
};

async fn get_login_user(github:Client) -> 
  Result<String> {

  let users = 
    github
    .get("https://api.github.com/user")
    .send()
    .await;

  let users_text = match users {
    Ok(resp) => resp.text().await,
    Err(e) => return Err(anyhow!(e)),
  };

  let users_json:Value = match users_text {
    Ok(text) => serde_json::from_str(&text)?,
    Err(e) => return Err(anyhow!(e)), 
  };

  let login = users_json["login"].as_str();

  let login = match login {
    Some(login) => login,
    None => return Err(anyhow!("param:login not found."))
  };

  Ok(login.into())
}

async fn get_repos(github:Client , login:&str)
  -> Result<Vec<Repository>> {
  let username = login;
  let repos = 
    github
    .get(format!("https://api.github.com/users/{username}/repos"))
    .send()
    .await;

  let repos_text = match repos {
    Ok(resp) => resp.text().await,
    Err(e) => return Err(anyhow!(e)),
  };

  let repos_json:Vec<Repository> = match repos_text {
    Ok(text) => serde_json::from_str(&text)?,
    Err(e) => return Err(anyhow!(e)), 
  };

  Ok(repos_json.into())
}

pub async fn get_traffic_info(
  github:Client,
  login:&str,
  repos:Vec<Repository>
) -> Result<Vec<TrafficInfo>> {
  let mut traffic_list: Vec<TrafficInfo> = vec![];

  let owner = login;
  for repo in repos {
    let repo_name = repo.name;

    let views_url = 
      format!("https://api.github.com/repos/{owner}/{repo_name}/traffic/views");
    let clones_url = 
      format!("https://api.github.com/repos/{owner}/{repo_name}/traffic/clones");


    let views = 
      github.clone()
      .get(views_url)
      .send();
    let clones = 
      github.clone()
      .get(clones_url)
      .send();

    let (result_views,result_clones) =
      future::join(views , clones).await;

    let views_text = 
    match result_views {
      Ok(resp) => resp.text(),
      Err(e) => return Err(anyhow!(e)),
    };
    let clones_text = 
    match result_clones {
      Ok(resp) => resp.text(),
      Err(e) => return Err(anyhow!(e)),
    };

    let (result_views_text, result_clones_text) =
      future::join(views_text, clones_text).await;

    let views_json:ViewsItem = 
    match result_views_text {
      Ok(text) => serde_json::from_str(&text)?,
      Err(e) => return Err(anyhow!(e)), 
    };
  
    let clones_json:ClonesItem = 
    match result_clones_text {
      Ok(text) => serde_json::from_str(&text)?,
      Err(e) => return Err(anyhow!(e)), 
    };

    let traffic_result = TrafficInfo {
      name: repo_name,
      url: repo.html_url,
      views_info: TrafficItem { 
        count: views_json.count, 
        uniques: views_json.uniques, 
        items: views_json.views },
      clones_info: TrafficItem {
        count: clones_json.count,
        uniques: clones_json.uniques,
        items: clones_json.clones,
      },
    };

    traffic_list.push(traffic_result);
  };

  Ok(traffic_list.into())
}

pub async fn main(personal_token:String) 
  -> Result<Vec<TrafficInfo>> {
    const USER_AGENT : &str = "user-agent-name";

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept",
     "application/vnd.github.v3+json".parse().unwrap());
    headers.insert("Authorization",
     format!("token {personal_token}").parse().unwrap());

  let github = reqwest::Client::builder()
    .user_agent(USER_AGENT)
    .default_headers(headers)
    .build();

  let github = match github {
    Ok(client) => client,
    Err(e) => return Err(anyhow!(e)),
  };

  let login = 
    get_login_user(github.clone()).await;

  let login = match login {
    Ok(login) => login,
    Err(e) => return Err(e),
  };

  let repos = 
    get_repos(github.clone(), &login).await;

  let repos = match repos {
    Ok(repos) => repos,
    Err(e) => return Err(e),
  };

  let traffic_info = 
    get_traffic_info(
      github.clone(),
      &login,
      repos)
    .await;

  let traffic_info = match traffic_info {
    Ok(info) => info,
    Err(e) => return Err(e),
  };

  Ok(traffic_info.into())
}