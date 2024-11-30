use serde::Serialize;

#[derive(Serialize)]
pub struct PageConfig {
  pub url: String,
  pub after: String,
  pub children: Vec<NodeConfig>
}

#[derive(Serialize)]
pub struct NodeConfig {
  pub selector: String,
  pub title: String,
  pub children: Vec<NodeConfig>
}

#[derive(Serialize)]
pub struct Page {
  pub url: String,
  pub after: String,
  pub children: Vec<Node>
}

#[derive(Serialize)]
pub struct Node {
  pub selector: String,
  pub title: String,
  pub content: String,
  pub children: Vec<Node>
}
