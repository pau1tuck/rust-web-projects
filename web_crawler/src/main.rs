use std::io::Read;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;

fn main() {
    let client = Client::new();
    let origin = "https://www.bbc.com/news/world-europe-57666389";
    let mut response = client.get(origin).send().unwrap();
    println!("Status for {}: {}", origin, response.status());

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    // println!("HTML: {}", &body[0..40]);
    Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}