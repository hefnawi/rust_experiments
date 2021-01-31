extern crate kuchiki;

use kuchiki::traits::*;
use ureq::Error;


fn main() {
    let url: &str = "https://www.creality3dofficial.com/collections/top-sell/products/ender-3-v2-3d-printer?variant=31883697487945";

    match ureq::get(url)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
        .call() {
        Ok(response) => {
            /* it worked */
            let html: String = response.into_string().unwrap();
            let result : String = select_css(html);
            println!("Result: {}", result);
        },
        Err(Error::Status(code, response)) => {
        /* the server returned an unexpected status
           code (such as 400, 500 etc) */
        }
        Err(_) => {
            /* some kind of io/transport error */
        }
    }

}

fn select_css(html: String) -> String {
    let css_selector = "option";

    let document = kuchiki::parse_html().one(html);

    for css_match in document.select(css_selector).unwrap() {
        // css_match is a NodeDataRef, but most of the interesting methods are
        // on NodeRef. Let's get the underlying NodeRef.
        let as_node = css_match.as_node();
        let text_node = as_node.first_child().unwrap();
        let text = text_node.as_text().unwrap().borrow();
        if text.contains("UK"){
            return text.to_string();
        }
        else {
            //println!("Other: {}", text.to_string());
        }
    }
    return "NULL".to_string();

}
