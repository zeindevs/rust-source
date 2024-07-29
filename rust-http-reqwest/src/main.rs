use reqwest;

fn main() {
    let resp = match reqwest::blocking::get("https://httpbin.org/ip") {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };
    println!("{}", resp);
}
