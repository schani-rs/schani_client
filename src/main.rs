extern crate hyper;
extern crate serde_json;

use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

use hyper::client::Client;
use serde_json::Value;

fn main() {
    let file_name = args().nth(1).expect("file name required");
    let file = File::open(file_name.to_owned()).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut buf = vec![];
    buf_reader
        .read_to_end(&mut buf)
        .expect("error reading image file");

    let client = Client::new();

    let url = hyper::Url::parse("http://localhost:8100/upload").expect("Could not parse URL");

    let mut resp = client
        .request(hyper::method::Method::Post, url)
        .body("name=test&user_id=100&camera=Nikon&latitude=40.1&longitude=10.3")
        .header(hyper::header::ContentType::form_url_encoded())
        .send()
        .expect("sending HTTP request failed");
    println!("Image data upload status: {}", resp.status);

    let mut resp_text = String::new();
    resp.read_to_string(&mut resp_text)
        .expect("could not read response text");
    let resp_json: Value = serde_json::from_str(&resp_text).expect("Could not read response JSON");
    println!("Successfully uploaded iamge {} and got ID {}",
             resp_json["name"],
             resp_json["id"]);

    let image_url = hyper::Url::parse(&format!("http://localhost:8100/upload/{}", resp_json["id"]))
        .expect("Could not parse URL");
    let mut req = hyper::client::request::Request::new(hyper::method::Method::Post, image_url)
        .expect("could not create image request");
    req.headers_mut()
        .set(hyper::header::ContentLength(buf.len() as u64));
    let mut str_req = req.start().expect("could not create streaming request");
    str_req.write_all(buf.as_slice());
    str_req.flush().expect("could not flush streaming request");
    let resp = str_req.send().expect("could not send streaming request");
    println!("Image upload status: {}", resp.status);
}
