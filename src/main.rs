use std::{fs, str};
use std::path::Path;

use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() -> std::io::Result<()> {
    let data_path = Path::new("data/complete-list.html");
    let f = fs::read(data_path)?;
    let docstr = str::from_utf8(&f).unwrap();

    let doc = Document::from(docstr);
    for node in doc.find(Class("results")) {
        println!("node {:?}", node);
    }
    Ok(())
}
