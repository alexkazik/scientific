//! # Macro for Scientific (Arbitrary precision scientific number)
//!
//! Allows the creation of [Scientific](struct@Scientific) number constants.
//!
//! ```
//! use scientific::Scientific;
//! use scientific_macro::Scientific;
//! let n1 = Scientific!(1e100);
//! let n2 = Scientific!(1e80);
//! assert_eq!(&n1 + &n2, Scientific!(1.00000000000000000001e100));
//! // An f64 has only a precision of about 15.9 digits, this are already 21.
//! ```

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Punct, Spacing, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use scientific::Scientific;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error, LitFloat, LitInt, Token};

// Helper: allow parsing Int or Float
enum LitIntOrLitFloat {
  LitInt(LitInt),
  LitFloat(LitFloat),
}

impl LitIntOrLitFloat {
  fn base10_digits(&self) -> &str {
    match self {
      LitIntOrLitFloat::LitInt(num) => num.base10_digits(),
      LitIntOrLitFloat::LitFloat(num) => num.base10_digits(),
    }
  }

  fn suffix(&self) -> &str {
    match self {
      LitIntOrLitFloat::LitInt(num) => num.suffix(),
      LitIntOrLitFloat::LitFloat(num) => num.suffix(),
    }
  }

  fn span(&self) -> Span {
    match self {
      LitIntOrLitFloat::LitInt(num) => num.span(),
      LitIntOrLitFloat::LitFloat(num) => num.span(),
    }
  }
}

impl Parse for LitIntOrLitFloat {
  fn parse(input: ParseStream) -> Result<Self, Error> {
    let lookahead = input.lookahead1();
    if lookahead.peek(LitInt) {
      input.parse().map(LitIntOrLitFloat::LitInt)
    } else if lookahead.peek(LitFloat) {
      input.parse().map(LitIntOrLitFloat::LitFloat)
    } else {
      Err(lookahead.error())
    }
  }
}

// Helper: Parse LitScientific (optional minus sign and LitIntOrLitFloat)
struct LitScientific {
  neg: Option<Token![-]>,
  num: LitIntOrLitFloat,
}

impl Parse for LitScientific {
  fn parse(input: ParseStream) -> Result<Self, Error> {
    Ok(LitScientific {
      neg: input.parse()?,
      num: input.parse()?,
    })
  }
}

// Helper: Convert to Tokens as a slice
struct AsSlice<'a>(&'a [u8]);

impl<'a> ToTokens for AsSlice<'a> {
  fn to_tokens(&self, stream: &mut TokenStream2) {
    let mut body_stream = TokenStream2::new();
    for d in self.0.iter() {
      d.to_tokens(&mut body_stream);
      body_stream.append(Punct::new(',', Spacing::Alone));
    }
    stream.append(Group::new(Delimiter::Bracket, body_stream));
  }
}

// Actual macro
/// Macro for Scientific (Arbitrary precision scientific number)
///
/// See the [module-level documentation](crate) for more details.
#[allow(non_snake_case)]
#[proc_macro]
pub fn Scientific(item: TokenStream) -> TokenStream {
  let LitScientific { neg, num } = parse_macro_input!(item as LitScientific);
  if !num.suffix().is_empty() {
    return TokenStream::from(Error::new(num.span(), "no suffix allowed").to_compile_error());
  }
  match Scientific::from_str(num.base10_digits()) {
    Err(e) => TokenStream::from(Error::new(num.span(), e).to_compile_error()),
    Ok(num) => {
      if num.is_zero() {
        quote!(scientific::Scientific::ZERO).into()
      } else {
        let neg = neg.is_some();
        let mantissa = AsSlice(num.as_raw_mantissa());
        let len = mantissa.0.len();
        let exponent = num.exponent();
        quote!(
          {
            const MANTISSA: [u8; #len] = #mantissa;
            scientific::Scientific::unchecked_non_zero_static_new(#neg, &MANTISSA, #exponent)
          }
        )
        .into()
      }
    }
  }
}
