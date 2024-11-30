use crate::models::Node;

mod models;
mod scraper;

#[tokio::main]
async fn main() {
    match scraper::scrape_articles().await {
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
