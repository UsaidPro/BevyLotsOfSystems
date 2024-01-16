use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse::Parse, parse_macro_input, Expr};

struct MacroInput {
    a: Expr
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let a = input.parse()?;
        Ok(Self {
            a: a,
        })
    }
}

/// Repeatedly adds systems 2000 times
///
/// Essentially repeats `app.add_systems(Update, (SystemA::<X>, SystemB::<X>, ...))` for X in 0..2000
///
/// Takes App as single input
#[proc_macro]
pub fn simulations(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as MacroInput);
    let appl = &input.a;

    let arguments = (0..2000).into_iter().enumerate().map(|(index, _arg)| quote! {
        #appl.add_systems(Startup, setup_physics::<#index>);
        #appl.add_systems(Update, (board_movement::<#index>, reset_simulation::<#index>.run_if(must_reset::<#index>)));
    });
    let gen = quote! {#(#arguments)*};
    TokenStream::from(gen)
}