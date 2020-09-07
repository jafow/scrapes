use std::{fs, str};
use std::path::Path;

use select::document::Document;
use select::predicate::{Predicate, Attr, Child, Class, Element, Name};

fn main() -> std::io::Result<()> {
    let data_path = Path::new("data/complete-list.html");
    let f = fs::read(data_path)?;
    let docstr = str::from_utf8(&f).unwrap();

    let doc = Document::from(docstr);
    for node in doc.find(Name("article")) {
        let article = Document::from(node.html().as_str());

        for (h2, tabl) in article.find(Name("h2")).zip(article.find(Class("results"))).take(2) {
            let xp = h2.text();
            dbg!(xp);
            let t = tabl.find(Name("td")).next().unwrap();
            let contact_numbers = t.first_child();
            let phone = contact_numbers.unwrap().text();
            let fax = t.next().unwrap().next().unwrap().text();
            dbg!(&phone);
            dbg!(&fax);

        }

        // for node in article.find(Name("h2")).take(2) {
            // let county_and_auditor_name = node;
            // dbg!(county_and_auditor_name.text());
            // county
            //
            // address 1
            //
            // address 2?
            //
            // city
            // zip
            // phone
            //
            // county website url
            //
            // email
            //
            // fax
            //
            // hours
            //
            // social
        // }
    }
    Ok(())
}
