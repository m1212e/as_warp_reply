extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(AsJsonReply)]
pub fn as_reply_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree representation
    let input = syn::parse_macro_input!(input as DeriveInput);

    // Get the name of the struct being derived for
    let name = input.ident;

    // Generate the implementation for the `into_response` method
    let expanded = quote! {
        impl warp::reply::Reply for #name {
            fn into_response(self) -> warp::reply::Response {
                match serde_json::to_string(&self) {
                    Ok(v) => warp::reply::with_status(v, warp::http::StatusCode::OK).into_response(),
                    Err(err) => {
                        log::error!("could not serialize response: {}", err);
                        warp::reply::with_status("Internal server error", warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                            .into_response()
                    }
                }
            }
        }
    };

    // Return the generated code as tokens
    TokenStream::from(expanded)
}
