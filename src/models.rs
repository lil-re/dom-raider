use std::collections::HashMap;
use serde::{Deserialize};

/// The description of a web page to scrape
///
/// The `pagination_selector` property is used to detect the pagination button and navigation to the
/// next page once the current page has been scraped
/// If pagination is no needed, just assign an empty String to the `pagination_selector`.
#[derive(Deserialize, Debug)]
pub struct Page {
  pub url: String,
  pub pagination_selector: String,
  pub children: Vec<Node>
}

/// A DOM node
#[derive(Deserialize, Debug)]
pub struct Node {
  pub title: String,
  pub selector: String,
  pub attribute: String,
  pub children: Vec<Node>
}

/// A CSV file item, this can be a sheet, line, or cell
///
/// A sheet should contain one or many lines through the `children` property.
/// A line should contain one or many cells through the `children` property.
/// A cell should not have children.
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
  Group,
  Line,
  Cell,
}

pub type SheetList = HashMap<String, Vec<FileItem>>;
