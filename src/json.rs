use std::fs;
use serde_json;
use crate::models::Page;

/// Read the content of a configuration file (e.g. `config.json`) and return a `Page`
///
/// The config file should have the same structure as a `Page` with `Node`
pub fn read_json() -> Result<Page, Box<dyn std::error::Error>> {
  let file_path = "config.json";
  let json_content = fs::read_to_string(file_path)?;
  let page: Page = serde_json::from_str(&*json_content)?;
  Ok(page)
}
