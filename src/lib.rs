use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, ConstParam, LitStr, Token};

struct Input {
    param: ConstParam,
    semi: Token![;],
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Input {
            param: input.parse()?,
            semi: input.parse()?,
        })
    }
}

/// Embed the response from an HTTP GET request as a const &str.
///
/// # Examples
/// ```rust
/// use from_http::from_http;
///
/// #[from_http("https://blog.rust-lang.org/2023/10/27/crates-io-non-canonical-downloads.html")]
/// const DATA: &str;
///
/// fn main() {
///     println!("{}", DATA);
/// }
/// ```
#[proc_macro_attribute]
pub fn from_http(attr: TokenStream, input: TokenStream) -> TokenStream {
    let data: LitStr = parse_macro_input!(attr as LitStr);
    let item: Input = parse_macro_input!(input as Input);
    let str_url = data.value();
    let result = reqwest::blocking::get(str_url);
    let _ = item.semi;

    match result {
        Ok(res) => match res.text() {
            Ok(txt) => {
                let generic_data = item.param;
                return quote! {
                    #generic_data = #txt;
                }
                .into();
            }
            Err(err) => {
                let msg = format!("Failed to get response body: {}", err.to_string());
                return syn::Error::new(data.span(), msg).to_compile_error().into();
            }
        },
        Err(err) => match err.status() {
            Some(stat) => {
                let msg = format!("Request failed with status: {}", stat);
                return syn::Error::new(data.span(), msg).to_compile_error().into();
            }
            None => {
                return syn::Error::new(data.span(), "Request failed with unknown status code.")
                    .to_compile_error()
                    .into();
            }
        },
    }
}
