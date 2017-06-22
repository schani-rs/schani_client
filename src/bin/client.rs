extern crate hyper;
extern crate serde_json;

use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use hyper::client::Client;
use serde_json::Value;

fn main() {
    let file_name = args().nth(1).expect("file name required");
    let sidecar_name = args().nth(2).expect("sidecar file name required");
    let file = File::open(file_name.to_owned()).expect("could not open file");
    let mut buf_image_reader = BufReader::new(file);
    let mut raw_buf = vec![];
    buf_image_reader.read_to_end(&mut raw_buf).expect(
        "error reading image file",
    );
    let sidecar_file = File::open(sidecar_name.to_owned()).expect("could not open file");
    let mut buf_sidecar_reader = BufReader::new(sidecar_file);
    let mut sidecar_buf = vec![];
    buf_sidecar_reader.read_to_end(&mut sidecar_buf).expect(
        "error reading image file",
    );

    let client = Client::new();

    let url = hyper::Url::parse("http://localhost:8100/upload").expect("Could not parse URL");

    let resp = client
        .request(hyper::method::Method::Post, url)
        .body(
            "name=test&user_id=100&camera=Nikon&latitude=40.1&longitude=10.3",
        )
        .header(hyper::header::ContentType::form_url_encoded())
        .send()
        .expect("sending HTTP request failed");
    println!("Image data upload status: {}", resp.status);

    let resp_json: Value = serde_json::from_reader(resp).expect("Could not read response JSON");
    println!(
        "Successfully uploaded iamge {} and got ID {}",
        resp_json["name"],
        resp_json["id"]
    );

    let image_url = hyper::Url::parse(&format!(
        "http://localhost:8100/upload/{}/raw",
        resp_json["id"]
    )).expect("Could not parse raw URL");
    let mut req = hyper::client::request::Request::new(hyper::method::Method::Post, image_url)
        .expect("could not create image request");
    req.headers_mut().set(hyper::header::ContentLength(
        raw_buf.len() as u64,
    ));
    let mut str_req = req.start().expect("could not create streaming request");
    str_req.write_all(raw_buf.as_slice());
    str_req.flush().expect("could not flush streaming request");
    let resp = str_req.send().expect("could not send streaming request");
    println!("Image upload status: {}", resp.status);

    let image_url = hyper::Url::parse(&format!(
        "http://localhost:8100/upload/{}/sidecar",
        resp_json["id"]
    )).expect("Could not parse sidecar URL");
    let mut req = hyper::client::request::Request::new(hyper::method::Method::Post, image_url)
        .expect("could not create image request");
    req.headers_mut().set(hyper::header::ContentLength(
        sidecar_buf.len() as u64,
    ));
    let mut str_req = req.start().expect("could not create streaming request");
    str_req.write_all(sidecar_buf.as_slice());
    str_req.flush().expect("could not flush streaming request");
    let resp = str_req.send().expect("could not send streaming request");
    println!("Image upload status: {}", resp.status);
}
