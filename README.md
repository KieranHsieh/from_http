# What is this?

This is a proof of concept rust proc-macro library that allows you to embed the response from an HTTP GET request into a string at compile time.

# Example
```rust
use from_http::from_http;

#[from_http("https://blog.rust-lang.org/2023/10/27/crates-io-non-canonical-downloads.html")]
const DATA: &str;

fn main() {
    println!("{}", DATA);
}
```

# Use Cases
- Generating bindings for protocol specifications at compile time.
- Embedding dynamic, but non-essential content.

# Further Development

This library could (but probably won't be) improved with the following features:
- Authentication support
- Support for methods other than GET
- Automatic serialization to HTML/XML/JSON
- Support for extracting a file from an archive.