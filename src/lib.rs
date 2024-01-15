#![allow(unused)]

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, ExprReference, Ident, Lit, Token,
};

enum Either {
    Lit(Literal),
    Ident(Ident),
    Expr(ExprReference),
}

impl Either {
    pub fn peek(input: &syn::parse::ParseStream) -> bool {
        // Cant figure out ParseStream::peek() for custom types so here is this
        // Peeks for either a `Lit`, `Ident`, or a `&Ident`
        input.peek(Lit) || input.peek(Ident) || input.peek(Token![&]) && (input.peek2(Ident))
    }
}

impl Parse for Either {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let out;

        if input.peek(Lit) {
            out = Self::Lit(input.parse()?)
        } else if input.peek(Ident) {
            out = Self::Ident(input.parse()?)
        } else {
            out = Self::Expr(input.parse()?)
        }

        Ok(out)
    }
}

impl ToTokens for Either {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Either::Lit(x) => x.to_tokens(tokens),
            Either::Ident(x) => x.to_tokens(tokens),
            Either::Expr(x) => x.to_tokens(tokens),
        }
    }
}

type Line = (Either, Option<Either>, Option<Ident>);
type Lines = Punctuated<Line, Token![,]>;

struct MacroInput {
    default_fmt: proc_macro2::TokenStream,
    lines: Lines,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let default_fmt = match input.peek(Ident) {
            true => {
                let default_fmt = input.parse::<Ident>()?;
                input.parse::<Token![;]>()?;
                quote!(#default_fmt.clone())
            }
            false => quote!(Default::default()),
        };

        let lines = input.parse_terminated(
            |input| {
                let text = input.parse::<Either>()?;

                let space = match Either::peek(&input) {
                    true => Some(input.parse()?),
                    false => None,
                };

                let fmt = match input.peek(Token![<]) {
                    true => {
                        input.parse::<Token![<]>()?;
                        let fmt = input.parse::<Ident>()?;
                        input.parse::<Token![>]>()?;
                        Some(fmt)
                    }
                    false => None,
                };

                Ok((text, space, fmt))
            },
            Token![,],
        )?;

        Ok(Self { default_fmt, lines })
    }
}

/// A macro for easily creating an epaint `LayoutJob`, which is rexported in egui. `LayoutJob` must be imported by a 'use' statement.
///
/// ## Usage
///
/// ```
/// layout!{
///     default_format;
///     text leading_space <text_format>,
///     text leading_space <text_format>,
///     ...
/// }
/// ```
///
/// Where:
/// - `default_format` must be identifier to a epaint `TextFormat`. Is optional. If present then semicolon is required after.
/// - `text` can be a expr or literal which must lead to a `&str`. Is required.
/// - `leading_space` can be an expr or a literal which must lead to a `f32`. Optional.
/// - `text_format` must be an identifier to a epaint `TextFormat`. Optional.
///
/// ## Notes
/// - If `default_format` is not present, then `Default::default()` is used.
/// - If for a certain line, a format is not present, then `default_format` is used.
/// - All the `TextFormats` are cloned.
///
/// ## Examples
/// ```
/// let fmt = TextFormat {
///     color: Color32::from_rgb(255, 0, 0),
///     ..Default::default()
/// }
/// let job: LayoutJob = layout!{
///     "Hello",
///     "World" 1.0 <fmt>
/// };
/// ```
#[proc_macro]
pub fn layout(input: TokenStream) -> TokenStream {
    let MacroInput { default_fmt, lines } = parse_macro_input!(input as MacroInput);

    let texts = lines.iter().map(|l: &Line| &l.0);

    let spaces = lines.iter().map(|l: &Line| &l.1).map(|space| match space {
        Some(s) => quote!(#s),
        None => quote!(0.0),
    });

    let fmts = lines.iter().map(|l: &Line| &l.2).map(|fmt| match fmt {
        Some(s) => quote!(#s.clone()),
        None => quote!(#default_fmt),
    });

    quote! {
        {
            let mut job = LayoutJob::default();
            #(
                job.append(#texts, #spaces, #fmts);
            )*
            job
        }
    }
    .into()
}
