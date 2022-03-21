fn main() {
    let resp = reqwest::blocking::get("http://localhost:8080/").unwrap();
    println!("{:?}", resp);
    println!("{:?}", resp.text());
}
