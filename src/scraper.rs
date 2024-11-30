use reqwest;
use scraper::{Element, Html, Selector};
use scraper::element_ref::ElementRef;
use crate::models::{Node, NodeConfig, PageConfig};

pub async fn use_web_scraper(page_config: PageConfig) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
  let nodes = match scrape_page(page_config).await {
    Ok(response) => response,
    Err(_) => panic!("Oops!!")
  };
  Ok(nodes)
}

pub async fn scrape_page(page_config: PageConfig) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
  let mut nodes: Vec<Node> = vec![];
  let mut url = page_config.url;

  loop {
    // sleep(Duration::from_millis(10000)).await;
    println!("{}", url);
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let body_selector = Selector::parse("body").unwrap();
    let body_element = document.select(&body_selector).next().unwrap();

    for node_config in page_config.children.iter() {
      match scrape_node(&node_config, body_element).await {
        None => {}
        Some(node) => {nodes.push(node)}
      };
    };

    let pagination_selector = Selector::parse(&*page_config.pagination).unwrap();
    let pagination_element = body_element.select(&pagination_selector).next();

    if let Some(element) = pagination_element {
      match scrape_pagination(element) {
        None => { break }
        Some(href) => {
          url = href;
          continue
        }
      };
    }
    break
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
    let content;

    if node_config.attribute.len() > 0 {
      content = node_element.value().attr(&node_config.attribute)?.to_string();
    } else {
      content = node_element.text().collect::<String>();
    }

    Some(Node {
      selector: String::from(&node_config.selector),
      title: String::from(&node_config.title),
      content,
      children: vec![]
    })
  }
}

pub fn scrape_pagination(element: ElementRef<'_>) -> Option<String> {
  let name = element.value().name();

  if name == "a" {
    Some(element.attr("href").unwrap().parse().unwrap())
  } else {
    scrape_pagination(element.parent_element()?)
  }
}
