use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields, Ident, Variant};

struct OpField {
    ident: Ident,
    fields: Fields,
}

pub fn derive_operations(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let op_fields = if let Data::Enum(DataEnum { variants, .. }) = data {
        variants
            .into_iter()
            .map(|Variant { ident, fields, .. }| OpField { ident, fields })
            .collect::<Vec<_>>()
    } else {
        unimplemented!()
    };

    let encoder = derive_encode_for_operation(&ident, &op_fields);

    quote! {
        #encoder
    }
    .into()
}

fn derive_encode_for_operation(ident: &Ident, op_fields: &[OpField]) -> proc_macro2::TokenStream {
    let encoders = op_fields
        .iter()
        .enumerate()
        .map(|(op_code, OpField { ident, fields })| match fields {
            Fields::Named(_) => todo!(),
            Fields::Unnamed(fields_unnamed) => {
                let fields = fields_unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| format_ident!("arg{idx}"))
                    .collect::<Vec<_>>();
                quote!(Self::#ident(#(#fields,)*) => {
                                    writer.write(&[#op_code as u8]);
                                    #(#fields.encode(writer);)*
                                }
                )
            }
            Fields::Unit => quote!(Self::#ident => writer.write(&[#op_code as u8])),
        });

    quote! {
        impl<Writer: Write> Encode<Writer> for #ident {
            fn encode(&self, writer: &mut Writer) {
                match self {
                    #(#encoders,)*
                }
            }
        }

        impl<Writer: Write> Encode<Writer> for [#ident] {
            fn encode(&self, writer: &mut Writer) {
                for op in self {
                    op.encode(writer);
                }
            }
        }
    }
}
