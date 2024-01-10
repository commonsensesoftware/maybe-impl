extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse2, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, ItemTrait, TraitBound,
    TraitBoundModifier, TypeParamBound,
};

use syn::{
    parse::{Parse, ParseStream},
    Path, Result,
};

struct TraitsAttribute {
    traits: Punctuated<Path, Comma>,
}

impl Parse for TraitsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            traits: input
                .parse_terminated(Path::parse, Comma)
                .expect("At least one type must be specified."),
        })
    }
}

/// Represents the metadata used to conditionally implement one or more traits.
/// 
/// # Arguments
/// 
/// * `traits` - one or more traits to add to a super trait.
/// 
/// # Remarks
/// 
/// This attribute is intended to be combined with the `cfg_attr` attribute to
/// conditionally implement the specified traits. The primary use case is to
/// conditionally add `Send` and `Sync`, but any user-specified traits are supported.
/// 
/// # Examples
/// 
/// The following trait only implements `Send` and `Sync` when the **async** feature
/// is activated.
/// 
/// ```
/// #[cfg_attr(feature = "async", maybe_impl::traits(Send,Sync))]
/// trait Foo {
///    fn bar(&self);
/// }
/// ```
#[proc_macro_attribute]
pub fn traits(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(_traits(
        TokenStream::from(metadata),
        TokenStream::from(input),
    ))
}

fn _traits(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let result = match parse2::<TraitsAttribute>(metadata) {
        Ok(attribute) => {
            if let Ok(mut trait_) = parse2::<ItemTrait>(TokenStream::from(input.clone())) {
                let traits = attribute.traits.iter().map(|path| {
                    TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: TraitBoundModifier::None,
                        lifetimes: None,
                        path: path.clone(),
                    })
                });
                trait_.supertraits.extend(traits);
                Ok(trait_.into_token_stream())
            } else {
                Err(Error::new(
                    input.span(),
                    "Attribute can only be applied to a trait.",
                ))
            }
        }
        Err(error) => Err(error),
    };

    match result {
        Ok(output) => output,
        Err(error) => error.to_compile_error().into(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn attribute_should_add_single_trait() {
        // arrange
        let metadata = TokenStream::from_str("Send").unwrap();
        let input = TokenStream::from_str("trait Foo { }").unwrap();
        let expected = "trait Foo : Send { }";

        // act
        let result = _traits(metadata, input);

        // assert
        assert_eq!(expected, result.to_string());
    }

    #[test]
    fn attribute_should_add_multiple_traits() {
        // arrange
        let metadata = TokenStream::from_str("Send, Sync").unwrap();
        let input = TokenStream::from_str(
            r#"
            trait IPityTheFoo {
                fn bar(&self);
            }
        "#,
        )
        .unwrap();
        let expected = concat!(
            "trait IPityTheFoo : Send + Sync { ",
            "fn bar (& self) ; ",
            "}",
        );

        // act
        let result = _traits(metadata, input);

        // assert
        assert_eq!(expected, result.to_string());
    }
}
