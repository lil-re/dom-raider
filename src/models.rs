use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Page {
  pub url: String,
  pub title: String,
  pub pagination_selector: String,
  pub children: Vec<Node>
}

#[derive(Deserialize, Debug)]
pub struct Node {
  pub selector: String,
  pub title: String,
  pub attribute: String,
  pub children: Vec<Node>
}

pub struct FileItem {
  pub title: String,
  pub content: String,
  pub children: Vec<FileItem>,
  pub item_type: FileItemType
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FileItemType {
  Sheet,
  Line,
  Cell,
}
