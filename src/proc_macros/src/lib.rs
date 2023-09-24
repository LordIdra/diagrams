use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse, Data, Ident};

#[proc_macro_derive(Increment)]
pub fn increment(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).expect("Failed to parse input.");
    let name = &ast.ident;
    if let Data::Enum(data_enum) = ast.data {
        let identifiers = data_enum.variants.iter().map(|variant| variant.ident.clone()).collect::<Vec<Ident>>();
        let mut identifiers_from = identifiers.clone();
        let last_identifier = identifiers_from.remove(identifiers.len()-1);
        let mut identifiers_to = identifiers.clone();
        identifiers_to.remove(0);

        quote! {
            impl super::Increment for #name {
                fn increment(&mut self) {
                    let new_value = match &self {
                        #(
                            Self::#identifiers_from => Self::#identifiers_to,
                        )*
                        Self::#last_identifier => panic!("Attempt to increment an enum past its last value"),
                    };
                    *self = new_value;
                }
            }
        }.into()

    } else {
        panic!("Increment can only be implemented for enums")
    }
}
