extern crate kuchiki;

use ureq;
use kuchiki::traits::*;

fn main() {
    ender_stock("https://www.creality3dofficial.com/collections/top-sell/products/ender-3-v2-3d-printer?variant=31883697487945");
}

fn ender_stock(url: &str) -> Result<(), ureq::Error>  {
    let body: String = ureq::get(url)
        .set("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0")
        .call()?
        .into_string()?;

    // println!("Page: {}", body);

    let css_selector = "[data-sku=3DPEnder3V2Y]";
    let document = kuchiki::parse_html().one(body);

    for css_match in document.select(css_selector).expect("") {
        let as_node = css_match.as_node();
        let text_node = as_node.first_child().expect("");
        let text = text_node.as_text().expect("").borrow();

        println!("{:?}", text);
    }
    Ok(())
}

