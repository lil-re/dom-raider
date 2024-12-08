use std::fs::File;
use csv::{Error, Writer};
use crate::models::{FileItem, FileItemType};

pub(crate) fn export_csv_file (file_items: Vec<FileItem>) -> Result<(), Error> {
  let mut wtr = csv::Writer::from_path("data.csv")?;

  for file_item in file_items {
    export_file_items(file_item, &mut wtr);
  }
  wtr.flush()?;
  Ok(())
}

fn export_file_items(file_item: FileItem, wtr: &mut Writer<File>) {
  if file_item.item_type == FileItemType::Sheet {
    let headers = export_headers(&file_item);
    &wtr.write_record(headers);

    for line in file_item.children {
      export_file_items(line, wtr)
    }
  } else if file_item.item_type == FileItemType::Line {
    let cells = export_cells(file_item);
    &wtr.write_record(cells);
  }
}

fn export_headers(file_item: &FileItem) -> Vec<&String> {
  let first_line = file_item.children.get(0);
  let mut headers = vec![];

  for cell in &first_line.unwrap().children {
    if cell.item_type == FileItemType::Cell {
      headers.push(&cell.title);
    }
  }
  headers
}

fn export_cells(file_item: FileItem) -> Vec<String> {
  let mut cells = vec![];

  for cell in file_item.children {
    if cell.item_type == FileItemType::Cell {
      cells.push(cell.content);
    }
  }
  cells
}
