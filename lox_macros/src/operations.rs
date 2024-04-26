use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields, Ident, Variant, Visibility};

use crate::utils::camel_to_snake;

struct OpField {
    ident: Ident,
    fields: Fields,
}

pub fn derive_operations(input: TokenStream) -> TokenStream {
    let DeriveInput {
        vis, ident, data, ..
    } = parse_macro_input!(input as DeriveInput);

    let op_fields = if let Data::Enum(DataEnum { variants, .. }) = data {
        variants
            .into_iter()
            .map(|Variant { ident, fields, .. }| OpField { ident, fields })
            .collect::<Vec<_>>()
    } else {
        unimplemented!()
    };

    let encoder = derive_encode_for_operation(&ident, &op_fields);

    let executor = get_executor(&vis, &ident, &op_fields);

    quote! {
        #encoder

        #executor
    }
    .into()
}

fn derive_encode_for_operation(ident: &Ident, op_fields: &[OpField]) -> proc_macro2::TokenStream {
    let encoders = op_fields
        .iter()
        .enumerate()
        .map(|(op_code, OpField { ident, fields })| {
            let op_code = op_code as u8;
            match fields {
                Fields::Named(_) => todo!(),
                Fields::Unnamed(fields_unnamed) => {
                    let fields = fields_unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| format_ident!("arg{idx}"))
                        .collect::<Vec<_>>();
                    quote! {
                        Self::#ident(#(#fields,)*) => {
                            writer.write(&[#op_code]);
                            #(#fields.encode(writer);)*
                        }
                    }
                }
                Fields::Unit => quote!(Self::#ident => writer.write(&[#op_code])),
            }
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

fn get_executor(
    vis: &Visibility,
    ident: &Ident,
    op_fields: &[OpField],
) -> proc_macro2::TokenStream {
    let trait_name = format_ident!("{}Executor", ident);
    let name = format_ident!("{}", camel_to_snake(&ident.to_string()));
    let executor_engine = format_ident!("execute_{name}");

    let (executor_fns, decoder_arms): (Vec<_>, Vec<_>) = op_fields
        .iter().enumerate()
        .map(|(op_code,OpField { ident, fields })| {
            let op_code = op_code as u8;
            let fn_name = format_ident!("{}", camel_to_snake(&ident.to_string()));

            let (params, (args, arg_names)): (Vec<_>, (Vec<_>, Vec<_>)) = fields
                .iter()
                .enumerate()
                .map(|(i, arg)| {
                    let name = arg.ident.clone().unwrap_or_else(|| format_ident!("arg{i}"));
                    let ty = &arg.ty;

                    (
                        quote!(#name: #ty),
                        (
                            quote! {
                                let (#name, size) = match <#ty>::decode(&buf[current..]) {
                                    Ok(v) => v,
                                    Err(err) => return Err(DecoderError::from_detail(current, *err).into()),
                                };
                                current += size;
                            },
                            quote!(#name)
                        ),
                    )
                })
                .unzip();

            (
                quote!(fn #fn_name(&mut self #(, #params)*) -> ExecutorResult<RuntimeError>;),
                quote!(#op_code => {
                    let mut current = next_code_index + 1;
                    #(#args)*
                    next_code_index = current;
                    executor.#fn_name(#(#arg_names,)*)?;
                }),
            )
        })
        .unzip();

    quote! {
        #vis trait #trait_name: Sized {
            #(#executor_fns)*
        }

        #vis fn #executor_engine<E: #trait_name>(executor: &mut E, buf: &[u8]) -> ExecutorResult<ExecutorError> {
            let mut next_code_index = 0;
            while next_code_index < buf.len() {
                let code = buf[next_code_index];
                match code {
                    #(#decoder_arms,)*
                    _ => unimplemented!()
                }
            }

            Ok(())
        }
    }
}
