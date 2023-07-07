use std::{
    io::{BufReader, Read},
    time::Instant,
};

use model::Index;

pub mod lexer;
pub mod model;
pub mod server;

fn main() -> Result<(), ()> {
    let mut index = Index::default();

    let urls = vec![
        "https://learn.microsoft.com/en-us/dotnet/api/system.xml.xmlreader?view=net-7.0",
        "https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/functional/pattern-matching",
        "https://learn.microsoft.com/en-us/dotnet/csharp/nullable-references",
        "https://learn.microsoft.com/en-us/dotnet/csharp/nullable-migration-strategies",
        "https://learn.microsoft.com/en-us/dotnet/csharp/methods",
    ];

    let start = Instant::now();
    for url in urls {
        println!("Fetching html of {url}");

        let contents = parse_html_page(url).chars().collect::<Vec<char>>();

        println!("\tIndexing contents of {url}");

        index.add_document(url, &contents);
    }
    println!(
        "Fetching html and collecting tokens took {ms}ms",
        ms = start.elapsed().as_millis()
    );
    println!("----------------------------------\n");
    let start = Instant::now();
    let results = index.search_query(&"   pattern   ".chars().collect::<Vec<char>>());

    println!(
        "Searching for \"pattern\" took {ms}ms\n",
        ms = start.elapsed().as_millis()
    );
    for result in results {
        println!("    {name} => {score}", name = result.0, score = result.1);
    }

    Ok(())
}

fn parse_html_page(url: &str) -> String {
    let response = ureq::get(url)
        .call()
        .map_err(|err| {
            eprintln!("ERROR: could not create request: {err}");
        })
        .unwrap();

    let reader = response.into_reader();
    let mut reader = BufReader::new(reader);

    let mut content_string = String::new();
    reader.read_to_string(&mut content_string).unwrap();
    let dom = tl::parse(&content_string, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let mut buffer = String::new();

    let body = dom
        .nodes()
        .iter()
        .find(|node| node.as_tag().map_or(false, |tag| tag.name() == "body"));

    if let Some(body) = body {
        buffer.push_str(&body.inner_text(parser));
        buffer.push(' ');
    } else {
        buffer.push_str(&format!("error {url}"));
    }

    buffer
}
