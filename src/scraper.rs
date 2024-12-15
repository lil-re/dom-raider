use std::collections::HashMap;
use std::error::Error;
use reqwest;
use scraper::{Element, Html, Selector};
use scraper::element_ref::{ElementRef, Select};
use crate::models::{FileItem, Node, Page, FileItemType, SheetList};

pub async fn use_web_scraper(page: Page) -> Result<HashMap<String, Vec<FileItem>>, Box<dyn std::error::Error>> {
  let base_url = String::from(&page.url);
  let mut url = String::from(&page.url);
  let mut sheets: SheetList = HashMap::new();

  loop {
    println!("{}", url);
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let body_selector = Selector::parse("body").unwrap();
    let body_element = document.select(&body_selector).next().unwrap();

    for node in page.children.iter() {
      match scrape_node(&node, body_element).await {
        None => {}
        Some(mut file_item) => {
          sheets.entry(file_item.title).or_insert(Vec::new()).append(&mut file_item.children);
        }
      };
    };

    match get_next_page_url(&page, &base_url, body_element) {
      Ok(next_page_url) => {
        url = next_page_url;
        continue
      }
      Err(error) => {
        println!("{} Stopping web scraper.", error);
        break
      }
    };
  }
  Ok(sheets)
}

/// If the `pagination_selector` is not empty, then try to retrieve a link to navigate to the next
/// page.
fn get_next_page_url(page: &Page, base_url: &String, body_element: ElementRef) -> Result<String, Box<dyn Error>> {
  if page.pagination_selector.len() > 0 {
    let pagination_selector = Selector::parse(&*page.pagination_selector).unwrap();
    let pagination_element = body_element.select(&pagination_selector).next();

    if let Some(element) = pagination_element {
      let found_url = find_next_page_url(element).unwrap_or_else(|| String::from(""));
      let new_url = build_next_page_url(base_url, found_url).unwrap_or_else(|| String::from(""));

      if new_url.to_owned().contains(&base_url.to_owned()) {
        Ok(new_url)
      } else {
        Err(Box::from("Cannot find next page URL."))
      }
    } else {
      Err(Box::from("Cannot find next page URL."))
    }
  } else {
    Err(Box::from("No configuration found for pagination."))
  }
}

/// Build a new url to continue web scraping.
fn build_next_page_url(base_url: &String, href: String) -> Option<String> {
  if href.contains("http") {
    // if the href is a complete URL, then it returns the href.
    Some(href)
  } else if href.len() > 0 {
    // Otherwise, it uses the `base_url` to build a new URL.
    let mut owned_url = base_url.to_owned();
    owned_url.push_str(&*href);
    Some(owned_url)
  } else {
    None
  }
}

/// Try to detect an element of the DOM to navigate to the next page.
pub fn find_next_page_url(element: ElementRef<'_>) -> Option<String> {
  let name = element.value().name();

  if name == "a" {
    // If the element is a link with a URL, it returns its content.
    Some(element.attr("href").unwrap().parse().unwrap())
  } else {
    // Otherwise, it will check the parent element and repeat operation.
    find_next_page_url(element.parent_element()?)
  }
}

/// Retrieve content of a `Node` and its children if necessary.
/// `parent_element` represent a parent node in the DOM.
pub async fn scrape_node(node: &Node, parent_element: ElementRef<'_>) -> Option<FileItem> {
  // Retrieve the elements inside the parent element matching the node selector from node config
  let node_selector = Selector::parse(&node.selector).unwrap();
  let mut node_elements = parent_element.select(&node_selector).clone();

  if node.children.len() > 0 {
    // If the node config has children, loop through all children nodes and retrieve their content
    scrape_node_children(node, &mut node_elements).await
  } else if let Some(node_element) = node_elements.next() {
    // Otherwise, Retrieve node content
    scrape_node_content(node, node_element)
  } else {
    None
  }
}

/// Loop through the children of Node and try to retrieve the content of each child.
/// `node_elements` represent children nodes in the DOM
async fn scrape_node_children(node: &Node, node_elements: &mut Select<'_, '_>) -> Option<FileItem> {
  let mut lines: Vec<FileItem> = vec![];

  // Loop through all elements
  for node_element in node_elements {
    let mut cells: Vec<FileItem> = vec![];
    // Loop through all children,
    for child in node.children.iter() {
      // Retrieve child nodes by using the child config and the node element as parent of the new node
      match Box::pin(scrape_node(&child, node_element)).await {
        None => {}
        Some(cell) => { cells.push(cell) }
      };
    }

    // If there are cells, it groups them into a `FileItem` of type `Line`
    if cells.len() > 0 {
      let line = FileItem {
        title: String::from("Line"),
        content: String::from(""),
        children: cells,
        item_type: FileItemType::Line
      };
      lines.push(line);
    }
  }

  // Finally, it groups lines into a `FileItem` of type `Group`
  Some(FileItem {
    title: String::from(&node.title),
    content: String::from(""),
    children: lines,
    item_type: FileItemType::Group
  })
}

/// Retrieve content of a child node
/// `node_element` represent one child node in the DOM
fn scrape_node_content(node: &Node, node_element: ElementRef) -> Option<FileItem> {
  let content;

  if node.attribute.len() > 0 {
    // If the `Node`.`attribute` property is not empty, then it retrieves the attribute's content.
    content = node_element.value().attr(&node.attribute)?.to_string();
  } else {
    // Otherwise, it returns the node's text.
    content = node_element.text().collect::<String>().trim().parse().unwrap();
  }

  // Returns a `FileItem` of type `Cell` without children.
  Some(FileItem {
    title: String::from(&node.title),
    content,
    children: vec![],
    item_type: FileItemType::Cell
  })
}
