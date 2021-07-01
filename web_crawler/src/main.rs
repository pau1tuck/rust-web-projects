use std::io::Read;
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let origin = "https://rolisz.ro/";
    let mut response = client.get(origin).send().unwrap();
    println!("Status for {}: {}", origin, response.status());

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    println!("HTML: {}", &body[0..40]);
}