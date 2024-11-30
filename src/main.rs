use crate::models::{Node, NodeConfig, PageConfig};

mod models;
mod scraper;

#[tokio::main]
async fn main() {
    let title_node_config = NodeConfig {
        title: String::from("Title"),
        selector: String::from("p.title"),
        attribute: String::from(""),
        children: vec![]
    };
    let description_node_config = NodeConfig {
        title: String::from("Description"),
        selector: String::from("p.teaser"),
        attribute: String::from(""),
        children: vec![]
    };
    let link_node_config = NodeConfig {
        title: String::from("Link"),
        selector: String::from("a.cta-big"),
        attribute: String::from("href"),
        children: vec![]
    };
    let logo_node_config = NodeConfig {
        title: String::from("Logo"),
        selector: String::from("img.logo"),
        attribute: String::from("src"),
        children: vec![]
    };
    let tile_node_config = NodeConfig {
        title: String::from("Software"),
        selector: String::from("article.descriptive-software-tile"),
        attribute: String::from(""),
        children: vec![title_node_config, description_node_config, link_node_config, logo_node_config]
    };
    let page_config = PageConfig {
        url: String::from("https://appvizer.fr/finance-comptabilite/comptabilite"),
        pagination: String::from("a svg.next"),
        children: vec![tile_node_config]
    };

    match scraper::use_web_scraper(page_config).await {
        Ok(nodes) => {
            for node in nodes.iter() {
                print_node(node)
            }
        },
        Err(_) => println!("Oops :-("),
    }
}

fn print_node(node: &Node) {
    if node.children.len() > 0 {
        for child_node in node.children.iter() {
            print_node(&child_node)
        }
    } else {
        println!("{} => {}", node.title, node.content);
    }
}
