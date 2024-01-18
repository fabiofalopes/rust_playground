use reqwest;
use scraper::{Html, Selector};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
#[allow(dead_code)]
struct Function {
    name: String,
    type_signature: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a URL as the argument");
        return Ok(());
    }

    let url = &args[1];
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut functions = Vec::new();

    for element in fragment.select(&selector) {
        let mut td_elements = element.select(&td_selector);
        if let (Some(name), Some(type_signature)) = (td_elements.next(), td_elements.next()) {
            let function = Function {
                name: name.text().collect::<Vec<_>>().join(""),
                type_signature: type_signature.text().collect::<Vec<_>>().join(""),
            };
            functions.push(function);
        }
    }

    let mut file = File::create("output.txt")?;

    for function in functions {
        println!("{:?}", function);
        writeln!(file, "{:?}", function)?;
    }
    
    Ok(())

}
