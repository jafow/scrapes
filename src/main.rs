use std::path::Path;
use std::{fs, str};
use std::fs::OpenOptions;
use std::io::Write;

use select::document::Document;
use select::predicate::{Class, Name};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip_code: String,
}

fn name_data(name: &str) -> (String, String) {
    // splits name data into County & Auditor name
    let s: Vec<&str> = name.split('-').collect();
    (String::from(s[0].trim()), String::from(s[1].trim()))
}

fn to_address(address: String) -> Option<Address> {
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
        zip_code: pieces[3].to_string(),
    })
}

#[test]
fn trim_address_test() {
    assert_eq!(
        Some(Address { street: "400 Public Square Ste 5".to_string(), city: "Greenfield".to_string(), state: "IA".to_string(), zip_code: "50849".to_string()}),
        to_address(String::from("\n\t\t\t\t\t\t\t\t\t\t400 Public Square Ste 5\u{a0}\n\t\t\t\t\t\t\tGreenfield\u{a0}IA\u{a0}50849\n\t\t\t\t\t")));

    assert_eq!(
        Some(Address { street: "400 Public Square, Ste 5".to_string(), city: "Greenfield".to_string(), state: "IA".to_string(), zip_code: "50849".to_string()}),
        to_address(String::from("\n\t\t\t\t\t\t\t\t\t\t400 Public Square, Ste 5\u{a0}\n\t\t\t\t\t\t\tGreenfield\u{a0}IA\u{a0}50849\n\t\t\t\t\t")));
}

fn main() -> std::io::Result<()> {
    // input
    let data_path = Path::new("data/complete-list.html");
    let f = fs::read(data_path)?;

    // output
    let output_data = Path::new("data/output.csv");
    let mut output = OpenOptions::new().create(true).append(true).open(output_data)?;
    let headers = "state,county,position,name,address,address2,email,fax,phone,website_of_county,source_url,maps_url";
    writeln!(&mut output, "{}", headers)?;

    // scrape
    let docstr = str::from_utf8(&f).unwrap();
    let doc = Document::from(docstr);

    for node in doc.find(Name("article")) {
        let article = Document::from(node.html().as_str());

        for (h2, table) in article
            .find(Name("h2"))
            .zip(article.find(Class("results")))
        {
            let mut data: String = String::new();
            let (county_name, auditor_name) = name_data(&h2.text());

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
            let mailing_address = to_address(td_selections[5].text()).unwrap();
            let physical_address = to_address(td_selections[6].text()).unwrap();
            let map_url = td_selections[7]
                .find(Name("a"))
                .take(1)
                .next()
                .unwrap()
                .attr("href")
                .unwrap();

            data.push_str(&format!("{}", "iowa"));
            data.push_str(&format!(",{}", county_name));
            data.push_str(&format!(",{}", "auditor"));
            data.push_str(&format!(",{}", auditor_name));
            // mailing
            data.push_str(&format!(",\"{}{}{}{}\"", mailing_address.street, mailing_address.city,  mailing_address.state, mailing_address.zip_code));
            // physical address
            data.push_str(&format!(",\"{}{}{}{}\"", physical_address.street, physical_address.city,  physical_address.state, physical_address.zip_code));

            data.push_str(&format!(",{}", email));

            data.push_str(&format!(",{}", fax));

            data.push_str(&format!(",{}", phone));

            data.push_str(&format!(",{}", website));
            data.push_str(&format!(",{}", "https://sos.iowa.gov/elections/auditors/auditor.asp?CountyID=00"));
            // google maps url
            data.push_str(&format!(",{}", String::from(map_url)));

            // write it to the output file
            writeln!(&mut output, "{}", data)?;
        }
    }
    Ok(())
}
