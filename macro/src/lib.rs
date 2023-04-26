use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse2, DeriveInput, Result, Error, MetaNameValue, Lit, LitStr, Expr, Meta};
use syn::parse::{Parse, ParseStream};
use quote::quote;

struct OpenAPIPathInput(LitStr);

impl Parse for OpenAPIPathInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name_value: MetaNameValue = input.parse()?;

        let expr = match name_value.value {
            Expr::Lit(expr_lit) => expr_lit,
            _ => return Err(Error::new(name_value.span(), "needs to be literal expression"))
        };

        let lit_str = match expr.lit {
            Lit::Str(lit_str) => lit_str,
            _ => return Err(Error::new(expr.span(), "needs to be literal expression"))
        };

        Ok(OpenAPIPathInput(lit_str))
    }
}

#[proc_macro_derive(SaxoRequest, attributes(saxo, openapi_path))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attribute = &input.attrs
        .iter()
        .nth(0)
        .filter(|a| a.path().segments.len() == 1 && a.path().segments[0].ident == "saxo")
        .expect("Expected saxo attribute");

    let parameters: OpenAPIPathInput = match &attribute.meta {
        Meta::List(list) => parse2(list.tokens.clone()).expect("Invalid saxo attribute!"),
        _ => panic!("Invalid saxo attribute!"),
    };

    let path_str = parameters.0.value();

    let expanded = quote! {
        use crate::SaxoRequest;

        impl SaxoRequest for Request {
            type ResponseType = Response;

            fn id(&self) -> &str {
                self.0
            }

            fn path() -> &'static str {
                #path_str
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(SaxoResponse)]
pub fn derive_saxo_response(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        use crate::SaxoResponse;

        impl SaxoResponse for Response { }

        impl fmt::Display for Response {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "SaxoResponse") // TODO: Automatically print all tokens would be cool
            }
        }
    };

    TokenStream::from(expanded)
}
