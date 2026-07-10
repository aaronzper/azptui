use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    if input_fn.sig.asyncness.is_some() {
        return syn::Error::new_spanned(
            &input_fn.sig.fn_token,
            "[azptui::component] async fns not supported",
        )
        .to_compile_error()
        .into();
    }

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let expanded = quote! {
        #[track_caller]
        #vis #sig {
            let mut __azptui__component_context =
                azptui::component::pre_render(::std::panic::Location::caller());

            let result = (|| #block)();

            azptui::component::post_render(__azptui__component_context);

            result
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn use_counter(_: TokenStream) -> TokenStream {
    quote! { __azptui__component_context.counter() }.into()
}

#[proc_macro]
#[track_caller]
pub fn use_state(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    quote! { __azptui__component_context.use_state(
        ::std::panic::Location::caller(),
        #args)
    }
    .into()
}

#[proc_macro]
#[track_caller]
pub fn on_event(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);
    quote! { __azptui__component_context.register_handler(
        ::std::panic::Location::caller(),
        #args)
    }
    .into()
}
