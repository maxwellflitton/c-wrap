extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse::Parse, parse::ParseStream,
    ItemFn, Ident, Token, Result, bracketed, LitBool
};


#[proc_macro_attribute]
pub fn wrap_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract function components
    let fn_inputs = &input_fn.sig.inputs;
    let fn_body = &input_fn.block.stmts;
    let fn_name = &input_fn.sig.ident;
    let fn_output = &input_fn.sig.output;


    let expanded = quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #fn_name(#fn_inputs) #fn_output {
            #(#fn_body)*
        }
    };
    return expanded.into()

    // let mut func = parse_macro_input!(item as ItemFn);

    // let vis      = &func.vis;
    // let attrs    = &func.attrs;
    // let inputs   = &func.sig.inputs;
    // let output   = &func.sig.output;
    // let orig_id  = &func.sig.ident;
    // let wrapper_id = Ident::new(&format!("{orig_id}_impl"), orig_id.span());

    // // Rename the original function so we can call it from the wrapper.
    // func.sig.ident = wrapper_id.clone();

    // quote! {
    //     #(#attrs)*                // keep doc comments, cfgs, etc.
    //     #func                     // the renamed original function

    //     // SAFETY: exposed as a C symbol with a unique name
    //     #[unsafe(no_mangle)]
    //     #vis unsafe extern "C" fn #orig_id(#inputs) #output {
    //         #wrapper_id(#(#inputs),*)
    //     }
    // }
    // .into()
}