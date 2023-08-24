use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let command_indent = &input.ident;
    let builder_name = format!("{}Builder", command_indent);
    let builder_indent = syn::Ident::new(&builder_name, command_indent.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!();
    };
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: std::option::Option<#ty> }
    });
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
                #name: self.#name.clone().ok_or(format!("{} not set", stringify!(#name)))?
        }
    });

    let empty_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: None }
    });
    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub fn #name(&mut self, #name: #ty) ->&mut Self{
            self.#name = Some(#name);
            self
        }}
    });
    let expanded = quote! {
        pub struct #builder_indent {
            #(#builder_fields,)*
        }
        impl #builder_indent {
            #(#builder_methods)*

            pub fn build(&self) -> std::result::Result<#command_indent, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#command_indent {
                    #(#build_fields,)*
                })
            }

        }
        impl #command_indent {
            pub fn builder() -> #builder_indent {
                #builder_indent {
                    #(#empty_fields,)*
                }
            }
        }

    };
    expanded.into()
}
