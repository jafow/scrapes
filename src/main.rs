use std::path::Path;
use std::{fs, str};

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Child, Class, Element, Name, Predicate};

fn name_data(name: &str) -> (String, String) {
    // splits name data into County & Auditor name
    let s: Vec<&str>= name.split("-").collect();
    (String::from(s[0].trim()), String::from(s[1].trim()))
}

fn main() -> std::io::Result<()> {
    let data_path = Path::new("data/complete-list.html");
    let f = fs::read(data_path)?;
    let docstr = str::from_utf8(&f).unwrap();

    let doc = Document::from(docstr);
    for node in doc.find(Name("article")) {
        let article = Document::from(node.html().as_str());

        for (h2, table) in article
            .find(Name("h2"))
            .zip(article.find(Class("results")))
            .take(1)
        {
            let mut data: String = String::new();
            let (county_name, auditor_name) = name_data(&h2.text());
            data.push_str(&format!(", {}", county_name));
            data.push_str(&format!(", {}", auditor_name));

            // get data elements holding contact and location info
            let td_selections = table
                .find(Name("td"))
                .into_selection()
                .iter()
                .collect::<Vec<_>>();
            let phone = td_selections[0].children().next().unwrap().text();
            let fax = td_selections[1].text();
            let email = td_selections[2].children().next().unwrap().text();
            let website = td_selections[3].children().next().unwrap().text();
            data.push_str(&format!(", {}", phone));
            data.push_str(&format!(", {}", fax));
            data.push_str(&format!(", {}", email));
            data.push_str(&format!(", {}", website));

            let hours = td_selections[4].text();
            let mailing = td_selections[5].text();
            let physical_address = td_selections[6].text();
            let map_url = td_selections[7].find(Name("a")).take(1).next().unwrap().attr("href").unwrap();
            data.push_str(&format!(", {}", hours));
            data.push_str(&format!(", {}", mailing));
            data.push_str(&format!(", {}", physical_address));
            data.push_str(&format!(", {}", String::from(map_url)));

            // write it to the output
            println!("data pre write {}", data);
        }
    }
    Ok(())
}
