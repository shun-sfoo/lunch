fn main() {
    let resp = reqwest::blocking::get("http://localhost:8555/").unwrap();
    println!("{:?}", resp);
    println!("{:?}", resp.text());
}
