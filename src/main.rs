use crate::models::{Node, NodeConfig, PageConfig};

mod models;
mod scraper;

#[tokio::main]
async fn main() {
    let name_node_config = NodeConfig {
        title: String::from("Name"),
        selector: String::from("a.profile__link"),
        attribute: String::from(""),
        children: vec![]
    };
    let symbol_node_config = NodeConfig {
        title: String::from("Symbol"),
        selector: String::from("span.profile__subtitle-name"),
        attribute: String::from(""),
        children: vec![]
    };
    let price_node_config = NodeConfig {
        title: String::from("Price"),
        selector: String::from("tr.table__row--full-width td:nth-of-type(2) div.valuta"),
        attribute: String::from(""),
        children: vec![]
    };
    let market_cap_node_config = NodeConfig {
        title: String::from("Market cap"),
        selector: String::from("tr.table__row--full-width td:nth-of-type(3) div.valuta"),
        attribute: String::from(""),
        children: vec![]
    };
    let change_node_config = NodeConfig {
        title: String::from("24h change"),
        selector: String::from("tr.table__row--full-width td:nth-of-type(4) div.change"),
        attribute: String::from(""),
        children: vec![]
    };
    let row_node_config = NodeConfig {
        title: String::from("Coin"),
        selector: String::from("tr.table__row--full-width"),
        attribute: String::from(""),
        children: vec![name_node_config, symbol_node_config, price_node_config, market_cap_node_config, change_node_config]
    };
    let page_config = PageConfig {
        url: String::from("https://coinranking.com"),
        title: String::from("CoinMarketCap"),
        // pagination_selector: String::from(r#"section.pagination img[alt="Next"]"#),
        pagination_selector: String::from(""),
        children: vec![row_node_config]
    };

    match scraper::use_web_scraper(page_config).await {
        Ok(page) => {
            for node in page.children.iter() {
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
