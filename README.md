# DOM Raider

DOM Raider is a web scraper written in Rust that allows you to extract data from a web page and export it to a CSV file.

## Table of Contents

- [Configuration](#configuration)
    - [Config File Structure](#config-file-structure)
    - [Pagination Configuration](#pagination-configuration)
    - [Node Configuration](#node-configuration)
- [How to Use](#how-to-use)
    - [Run the Scraper](#run-the-scraper)
    - [Exported Results](#exported-results)
- [Troubleshooting](#troubleshooting)

## Configuration

The scraper needs a `config.json` file that defines the structure of the web page, including the URL to scrape, any pagination settings, and the nodes to scrape within the page.

### Config File Structure

The configuration file (`config.json`) should have the following structure:

```json
{
  "url": "https://example.com",  
  "pagination_selector": ".pagination .next",  
  "children": [
    {
      "title": "Group 1",
      "selector": ".group-1",
      "attribute": "data-group",
      "children": [
        {
          "title": "Title",
          "selector": ".title",
          "attribute": "data-title",
          "children": []
        },
        {
          "title": "Description",
          "selector": ".description",
          "attribute": "data-description",
          "children": []
        }
      ]
    },
    {
      "title": "Group 2",
      "selector": ".group-2",
      "attribute": "data-group",
      "children": [
        {
          "title": "Title",
          "selector": ".title",
          "attribute": "data-title",
          "children": []
        }
      ]
    }
  ]
}
```

### Pagination Configuration

The property `pagination_selector` is used to specify the CSS selector for the pagination link to the next page. If no 
pagination is needed, this can be an empty string (`""`).

Example:

```
"pagination_selector": ".pagination .next"
```

This would select the "next" button in the pagination area to navigate through multiple pages. If this is empty, the 
scraper will only scrape the first page and will not look for additional pages.

### Node Configuration

A "node" in the configuration file corresponds to an element or a group of elements on the page that the scraper will 
extract. Each node can have the following properties:

* `title`: A label for the node that will be used as a column or group name in the exported CSV.
* `selector`: The CSS selector to locate the element or group of elements to scrape.
* `attribute`: The attribute from the element to scrape. If left empty, the scraper will extract the text content of the 
* element.
* `children`: A list of child nodes that are contained within this node.

Example:

```json
{
  "title": "Group 1",
  "selector": ".group-1",
  "attribute": "",
  "children": [
    {
      "title": "Title",
      "selector": ".title",
      "attribute": "",
      "children": []
    },
    {
      "title": "Description",
      "selector": ".description",
      "attribute": "",
      "children": []
    }
  ]
}
```

A node can have nested child nodes. These child nodes will be recursively scraped and grouped in the output file. For 
example, the above configuration will extract each nested elements in the element with `class="group-1"` and extract 
titles and descriptions as separate cells in the output file.

## How to Use

### Run the Scraper

You will need to have Rust and cargo installed on your system.

To run the scraper, follow these steps:

* `git clone https://github.com/lil-re/dom-raider.git`

* `cd dom-raider`

* `cargo build`

* Add a `config.json` file in the root of the project.

* `cargo run`

### Exported Results

The scraper will export the results in CSV format. Each group of nodes will be exported as a separate CSV file, named 
after the group title. For example, if the configuration defines a node with the title "Group 1", the scraper will 
create a file called Group 1.csv.

The exported CSV file will contain:
* `Headers`: The first row will contain the column titles, derived from the title property of each node. For example, if 
* you have nodes with titles like "Title" and "Description", they will appear as headers in the CSV.
* `Rows`: Each row in the CSV will correspond to the data extracted from the elements. If the node has children (e.g., 
* "Line" or "Cell"), the rows will be populated with the corresponding content.

For example, if the following nodes are scraped:

```json
{
  "title": "Group 1",
  "children": [
    {
      "title": "Title",
      "selector": ".title",
      "attribute": "",
      "children": []
    },
    {
      "title": "Description",
      "selector": ".description",
      "attribute": "",
      "children": []
    }
  ]
}
```

The output CSV file Group 1.csv might look like this:  
```csv
Title,Description
Item 1,A description
Item 2,Another description
Item 3,More details
```

## Troubleshooting

If the config.json file is incorrectly formatted or missing required fields, the scraper will 
panic and print an error message.  
If no pagination is configured and the page has multiple pages, the scraper will only process the 
first page.  
If the scraper cannot find elements based on the specified CSS selectors, it will skip those nodes and 
move on to the next.  
