// find_by_or!{ Person -> people::[name:String || company_name:String] }
use proc_macro2::{Ident, TokenStream};
use proc_macro2::TokenTree::Ident;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn find_by_or(input: TokenStream) -> TokenStream {
    let DbOpByOrBy {
        model,
        table,
        bracket_token,
        content,
    } = parse_macro_input!(input as DbOpByOrBy);
    let (name1, name2) = (content.name1, content.name2);
    let (ty1, ty2) = (content.ty1, content.ty2);
    let fn_name = format!("find_by_{}_or_{}", name1, name2);
    let fn_name = Ident::new(&fn_name, proc_macro2::Span::call_site());

    let expanded = quote! {
      impl #model {
            pub fn #fn_name(conn: &PgConnection, #name1: #ty1, #name2: ty2) -> QueryResult<#model> {
                #table::table
                .filter(#table::dsl::#name1.eq(#name1))
                .or_filter(#table::dsl::#name2.eq(#name2))
                .get_result(conn)
            }
        }
    };
    TokenStream::from(expanded)
}