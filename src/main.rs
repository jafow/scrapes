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
    let headers = "address_1,state,county,Field 23,location_type,address2,city,state_2,zip,phone,latitude,longitude,has_dropoff,has_phone_number,county_website,validate_url,email,fax,social,inactive,hours,rules,notes,state and counties and county reps copy";
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
            let (county_name, _) = name_data(&h2.text());

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
    
            // "address_1,state,county,Field 23,location_type,address2,city,state_2,zip,phone,latitude,longitude,has_dropoff,has_phone_number,county_website,validate_url,email,fax,social,inactive,hours,rules,notes,state and counties and county reps copy";
            data.push_str(&format!("\"{}\"", physical_address.street));
            data.push_str(&format!(",{}", "iowa"));
            data.push_str(&format!(",{}", county_name));

            // Field 23
            data.push_str(&format!(",{}", ""));
            data.push_str(&format!(",{}", "auditor"));

            data.push_str(&format!(",\"{}\"", mailing_address.street));
            data.push_str(&format!(",{}", mailing_address.city));

            data.push_str(&format!(",{}", "IA"));

            data.push_str(&format!(",{}", mailing_address.zip_code));

            data.push_str(&format!(",{}", phone));

            data.push_str(&format!(",{}", 0.0_f32));
            data.push_str(&format!(",{}", 0.0_f32));

            // has dropoff
            data.push_str(&format!(",{}", ""));
            data.push_str(&format!(",{}", true));

            data.push_str(&format!(",{}", website));
            data.push_str(&format!(",{}", "https://sos.iowa.gov/elections/auditors/auditor.asp?CountyID=00"));

            data.push_str(&format!(",{}", email));
            data.push_str(&format!(",{}", fax));

            // social
            data.push_str(&format!(",{}", ""));
            // inactive
            data.push_str(&format!(",{}", ""));
            // hours
            data.push_str(&format!(",{}", ""));
            // rules
            data.push_str(&format!(",{}", ""));
            // notes
            data.push_str(&format!(",{}", ""));
            // state and counties and county reps copy
            data.push_str(&format!(",{}", ""));

            // write it to the output file
            writeln!(&mut output, "{}", data)?;
        }
    }
    Ok(())
}
