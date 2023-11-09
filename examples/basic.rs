use from_http::from_http;

#[from_http("https://blog.rust-lang.org/2023/10/27/crates-io-non-canonical-downloads.html")]
const DATA: &str;

fn main() {
    println!("{}", DATA);
}
