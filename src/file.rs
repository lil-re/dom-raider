use crate::models::FileItem;

pub(crate) fn export_csv_file (file_items: Vec<FileItem>) -> Result<String, Box<String>> {
  for file_item in file_items {
    export_file_item_content(file_item);
  }
  Ok(String::from("Ok"))
}

fn export_file_item_content(file_item: FileItem) {
  if file_item.children.len() > 0 {
    println!("{:?}", file_item.item_type);
    for child_file_item in file_item.children {
      export_file_item_content(child_file_item)
    }
  } else {
    println!("{:?} => {}", file_item.item_type, file_item.content);
  }
}
