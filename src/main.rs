use std::path::Path;
use std::{fs, str};

use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Child, Class, Element, Name, Predicate};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

fn name_data(name: &str) -> (String, String) {
    // splits name data into County & Auditor name
    let s: Vec<&str> = name.split("-").collect();
    (String::from(s[0].trim()), String::from(s[1].trim()))
}

fn trim_address<'a>(address: String) -> Option<Address> {
    let pieces = address
        .split_terminator('\u{a0}')
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    if pieces.len() < 4 {
        println!("missing address data: {:?}", &pieces);
        return None;
    }
    Some(Address {
        street: pieces[0].to_string(),
        city: pieces[1].to_string(),
        state: pieces[2].to_string(),
        zip: pieces[3].to_string(),
    })
}

#[test]
fn trim_address_test() {
    assert_eq!(
        // Some(String::from("400 Public Square Ste 5 Greenfield IA 55555")),
        Some(Address { street: "400 Public Square Ste 5".to_string(), city: "Greenfield".to_string(), state: "IA".to_string(), zip: "50849".to_string()}),
        trim_address(String::from("\n\t\t\t\t\t\t\t\t\t\t400 Public Square Ste 5\u{a0}\n\t\t\t\t\t\t\tGreenfield\u{a0}IA\u{a0}50849\n\t\t\t\t\t")));
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
            let mailing_address = td_selections[5].text();
            let physical_address = trim_address(td_selections[6].text());
            let map_url = td_selections[7]
                .find(Name("a"))
                .take(1)
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
            println!("mailng {:?}", mailing_address);
            data.push_str(&format!(", {}", hours));
            // data.push_str(&format!(", {}", mailing_address));
            // data.push_str(&format!(", {}", physical_address));
            data.push_str(&format!(", {}", String::from(map_url)));

            // write it to the output
            println!("data pre write {}", data);
        }
    }
    Ok(())
}
