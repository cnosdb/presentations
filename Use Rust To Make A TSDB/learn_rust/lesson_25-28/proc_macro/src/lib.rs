use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, Fields, parse_macro_input};
use syn::__private::TokenStream2;
use syn::DeriveInput;
use proc_macro2::Ident;
use syn::Type;

// #[proc_macro_derive(Builder)]
// pub fn derive_builder(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let ident = input.ident;
//     let builder_ident = format_ident!("{}Builder", ident);
//     quote!(
//         impl #ident {
//             pub fn builder() -> #builder_ident {
//                 #builder_ident
//             }
//         }
//
//         pub struct #builder_ident;
//     ).into()
// }

fn map_fields<F>(fields: &Fields, func: F) -> TokenStream2
where
    F: FnMut((&Ident, &Type)) -> TokenStream2,
{
    TokenStream2::from_iter(
        fields
            .iter()
            .map(|field|(field.ident.as_ref().unwrap(), &field.ty))
            .map(func)
    )
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let builder_ident = format_ident!("{}Builder", ident);
    if let Data::Struct(data_struct) = input.data {
        let fields = data_struct.fields;
        if  matches!(fields, Fields::Named(_)){
            let builder_fields = map_fields(&fields, |(ident, ty)| quote!(#ident: Option<#ty>, ));
            let builder_set_method = map_fields(&fields, |(ident, ty)| quote!(
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
            ));
            let builder_let = map_fields(&fields, |(ident, _)| quote!(
                let #ident = self.#ident.ok_or(())?;
            ));
            let build_value = map_fields(&fields, |(ident, _)| quote!(#ident, ));
            return quote!(
                impl #ident {
                    pub fn builder() -> #builder_ident {
                        #builder_ident::default()
                    }
                }

                #[derive(Default)]
                pub struct #builder_ident {
                    #builder_fields
                }

                impl #builder_ident {
                    #builder_set_method

                    pub fn build(self) -> Result<#ident, ()> {
                        #builder_let
                        Ok(#ident { #build_value })
                    }
                }
            ).into()

        }
    }
    quote!().into()
}




#[cfg(test)]
mod tests {
}
