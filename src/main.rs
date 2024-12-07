use crate::models::{FileItem, Node, Page};

mod models;
mod scraper;
mod file;

#[tokio::main]
async fn main() {
    let name_node_config = Node {
        title: String::from("Name"),
        selector: String::from("span.coin-profile__name"),
        attribute: String::from(""),
        children: vec![]
    };
    let symbol_node_config = Node {
        title: String::from("Symbol"),
        selector: String::from("span.coin-profile__symbol"),
        attribute: String::from(""),
        children: vec![]
    };
    let price_node_config = Node {
        title: String::from("Price"),
        selector: String::from("div.coins-table table tbody tr td:nth-of-type(4) real-time-price"),
        attribute: String::from(""),
        children: vec![]
    };
    let market_cap_node_config = Node {
        title: String::from("Market cap"),
        selector: String::from("div.coins-table table tbody tr td:nth-of-type(4) div.table__sub-item"),
        attribute: String::from(""),
        children: vec![]
    };
    let change_node_config = Node {
        title: String::from("24h change"),
        selector: String::from("span.change__percentage"),
        attribute: String::from(""),
        children: vec![]
    };
    let row_node_config = Node {
        title: String::from("Coin"),
        selector: String::from("div.coins-table table tbody tr"),
        attribute: String::from(""),
        children: vec![name_node_config, symbol_node_config, price_node_config, market_cap_node_config, change_node_config]
    };
    let page_config = Page {
        url: String::from("https://coinranking.com/coins"),
        title: String::from("CoinMarketCap"),
        pagination_selector: String::from(r#"a.pagination__next"#),
        // pagination_selector: String::from(""),
        children: vec![row_node_config]
    };

    match scraper::use_web_scraper(page_config).await {
        Ok(file_items) => {
          file::export_csv_file(file_items).expect("TODO: panic message");
        },
        Err(_) => println!("Oops :-("),
    }
}
