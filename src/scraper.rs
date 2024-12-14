use std::error::Error;
use reqwest;
use scraper::{Element, Html, Selector};
use scraper::element_ref::{ElementRef, Select};
use crate::models::{FileItem, Node, Page, FileItemType};

pub async fn use_web_scraper(page_config: Page) -> Result<Vec<FileItem>, Box<dyn std::error::Error>> {
  let file_items = match scrape_page(&page_config).await {
    Ok(response) => response,
    Err(_) => panic!("Oops!!")
  };
  Ok(file_items)
}

pub async fn scrape_page(page: &Page) -> Result<Vec<FileItem>, Box<dyn std::error::Error>> {
  let base_url = String::from(&page.url);
  let mut url = String::from(&page.url);
  let mut file_items: Vec<FileItem> = vec![];

  loop {
    println!("{}", url);
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let body_selector = Selector::parse("body").unwrap();
    let body_element = document.select(&body_selector).next().unwrap();

    for node in page.children.iter() {
      match scrape_node(&node, body_element).await {
        None => {}
        Some(file_item) => {file_items.push(file_item)}
      };
    };

    match get_next_page_url(page, &base_url, body_element) {
      Ok(next_page_url) => {
        url = next_page_url;
        continue
      }
      Err(..) => { break }
    };
  }
  Ok(file_items)
}

fn get_next_page_url(page: &Page, base_url: &String, body_element: ElementRef) -> Result<String, Box<dyn Error>> {
  if page.pagination_selector.len() > 0 {
    let pagination_selector = Selector::parse(&*page.pagination_selector).unwrap();
    let pagination_element = body_element.select(&pagination_selector).next();

    if let Some(element) = pagination_element {
      let url = match scrape_pagination(element) {
        None => String::from(""),
        Some(href) => {
          if href.contains("http") {
            href
          } else {
            let mut owned_url = base_url.to_owned();
            owned_url.push_str(&*href);
            owned_url
          }
        }
      };

      if url.to_owned().contains(&base_url.to_owned()) {
        Ok(url)
      } else {
        Err(Box::from("Cannot find next page URL"))
      }
    } else {
      Err(Box::from("Cannot find next page URL"))
    }
  } else {
    Err(Box::from("Cannot find next page URL"))
  }
}

pub async fn scrape_node(node: &Node, parent_element: ElementRef<'_>) -> Option<FileItem> {
  // Retrieve the elements inside the parent element matching the node selector from node config
  let node_selector = Selector::parse(&node.selector).unwrap();
  let mut node_elements = parent_element.select(&node_selector).clone();

  // If the node config has children
  if node.children.len() > 0 {
    scrape_node_children(node, &mut node_elements).await
  } else {
    scrape_node_content(node, &mut node_elements)?
  }
}

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

  Some(FileItem {
    title: String::from(&node.title),
    content: String::from(""),
    children: lines,
    item_type: FileItemType::Sheet
  })
}

fn scrape_node_content(node: &Node, node_elements: &mut Select) -> Option<Option<FileItem>> {
  // Otherwise, Retrieve node content
  Some(if let Some(node_element) = node_elements.next() {
    let content;

    if node.attribute.len() > 0 {
      content = node_element.value().attr(&node.attribute)?.to_string();
    } else {
      content = node_element.text().collect::<String>().trim().parse().unwrap();
    }

    Some(FileItem {
      title: String::from(&node.title),
      content,
      children: vec![],
      item_type: FileItemType::Cell
    })
  } else {
    None
  })
}

pub fn scrape_pagination(element: ElementRef<'_>) -> Option<String> {
  let name = element.value().name();

  if name == "a" {
    Some(element.attr("href").unwrap().parse().unwrap())
  } else {
    scrape_pagination(element.parent_element()?)
  }
}
