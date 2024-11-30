use reqwest;
use scraper::{Html, Selector};
use scraper::node::Element;
use scraper::element_ref::ElementRef;
use serde::de::Unexpected::Str;
use crate::models::{Node, NodeConfig, PageConfig};

pub async fn scrape_articles() -> Result<Vec<Node>, Box<dyn std::error::Error>> {
  let title_node_config = NodeConfig {
    title: String::from("Title"),
    selector: String::from("p.title"),
    children: vec![]
  };
  let description_node_config = NodeConfig {
    title: String::from("Description"),
    selector: String::from("p.teaser"),
    children: vec![]
  };
  let teaser_node_config = NodeConfig {
    title: String::from("Software"),
    selector: String::from("div.title-teaser"),
    children: vec![title_node_config, description_node_config]
  };
  let page_config = PageConfig {
    url: String::from("https://appvizer.fr/finance-comptabilite/comptabilite"),
    after: String::from("a.cim-label"),
    children: vec![teaser_node_config]
  };

  let response = reqwest::get(page_config.url).await?.text().await?;
  let document = Html::parse_document(&response);
  let selector = Selector::parse("body").unwrap();
  let body = document.select(&selector).next().unwrap();
  let mut nodes: Vec<Node> = vec![];

  for node_config in page_config.children.iter() {
    match scrape_node(&node_config, body).await {
      None => {}
      Some(node) => {nodes.push(node)}
    }
  }
  Ok(nodes)
}


pub async fn scrape_node(node_config: &NodeConfig, parent_element: ElementRef<'_>) -> Option<Node> {
  // Retrieve the elements inside the parent element matching the node selector from node config
  let node_selector = Selector::parse(&node_config.selector).unwrap();
  let mut node_elements = parent_element.select(&node_selector).clone();

  // If the node config has children
  if node_config.children.len() > 0 {
    let mut node_children: Vec<Node> = vec![];

    // Loop through all elements
    for node_element in node_elements {
      // Loop through all children,
      for child_config in node_config.children.iter() {
        // Retrieve child nodes by using the child config and the node element as parent of the new node
        let child_node = Box::pin(scrape_node(&child_config, node_element)).await;
        node_children.push(child_node?)
      }
    }

    Some(Node {
      selector: String::from(&node_config.selector),
      title: String::from(&node_config.title),
      content: String::from(""),
      children: node_children
    })
  } else {
    // Otherwise, Retrieve node content
    let node_element = node_elements.next().unwrap();
    let content = node_element.text().collect::<String>();
    // println!("selector => {}", &node_config.selector);
    // println!("content => {}", content);

    Some(Node {
      selector: String::from(&node_config.selector),
      title: String::from(&node_config.title),
      content,
      children: vec![]
    })
  }
}
