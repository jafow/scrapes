// extern crate async_std;
// extern crate libflate;
// extern crate surf;

use std::error::Error;

use async_std::task;
use surf;

use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    task::block_on(async {
        let mut res = surf::get("https://sos.iowa.gov/elections/auditors/auditor.asp?CountyID=99")
            .set_header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .set_header("Accept-Language", "en-US,en;q=0.5")
            .set_header("Cache-Control", "no-cache")
            .set_header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:80.0) Gecko/20100101 Firefox/80.0")
            .set_header("Host", "sos.iowa.gov")
            .recv_string()
            .await?;
        let doc = Document::from(res);

        for node in doc.find(Attr
        Ok(())
    })
}
