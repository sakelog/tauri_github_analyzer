#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct Repository {
  pub html_url: String,
  pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(serde::Deserialize,serde::Serialize)]
pub struct TrafficInfo {
  pub name: String,
  pub url: String,
  pub views_info : TrafficItem,
  pub clones_info: TrafficItem,
}

#[derive(Debug)]
#[derive(serde::Deserialize,serde::Serialize)]
pub struct TrafficItem {
  pub count: i64,
  pub uniques: i64,
  pub items: Vec<Traffic>
}
#[derive(Debug)]
#[derive(serde::Deserialize,serde::Serialize)]
pub struct ViewsItem {
  pub count: i64,
  pub uniques: i64,
  pub views: Vec<Traffic>
}
#[derive(Debug)]
#[derive(serde::Deserialize,serde::Serialize)]
pub struct ClonesItem {
  pub count: i64,
  pub uniques: i64,
  pub clones: Vec<Traffic>
}

#[derive(Debug)]
#[derive(serde::Deserialize,serde::Serialize)]
pub struct Traffic {
  pub count: i64,
  pub timestamp: String,
  pub uniques: i64,
}