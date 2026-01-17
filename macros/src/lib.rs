use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn handler(args: proc_macro::TokenStream, input: proc_macro::TokenStream)
               -> proc_macro::TokenStream
{
    let method = parse_macro_input!(args as LitStr);
    let func = parse_macro_input!(input as ItemFn);

    let method_value = method.value();
    let fn_name = &func.sig.ident;

    let expanded = quote::quote! {
        #func
        inventory::submit! {
            common::registry::HandlerEntry {
                method: #method_value,
                func: #fn_name,
            }
        }
    };

    expanded.into()
}