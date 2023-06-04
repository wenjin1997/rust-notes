use proc_macro2::Ident;
use syn::{bracketed, token, Token, Type};
use syn::parse::{Parse, ParseStream};
use super::*;
#[test]
fn main() {
    pub struct DbOpByOrBy {
        pub model: Type,
        pub table: Ident,
        pub bracket_token: token::Bracket,
        pub content: FieldContentOr,
    }

    pub struct FieldContentOr {
        pub name1: Ident,
        pub ty1: Type,
        pub name2: Ident,
        pub ty2: Type,
    }

    impl Parse for DbOpByOrBy {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            let model: Type = input.parse()?;
            input.parse::<Token![->]>()?;
            let table: Ident = input.parse()?;
            input.parse::<Token![::]>()?;
            let bracket_token = bracketed!(content in input);
            let content = content.parse()?;
            Ok(DbOpByOrBy {
                model,
                table,
                bracket_token,
                content,
            })
        }
    }
    vec![]

    impl Parse for FieldContentOr {
        fn parse(input: ParseStream) -> Result<Self> {
            let name1: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty1: Type = input.parse()?;
            input.parse::<Token![||]>()?;
            let name2: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty2: Type = input.parse()?;
            Ok(FieldContentOr {
                name1,
                ty1,
                name2,
                ty2,
            })
        }
    }
}