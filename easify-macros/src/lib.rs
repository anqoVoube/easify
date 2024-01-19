
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, LitInt, parse::Parse, parse::ParseStream, Result, Token};


/// A procedural macro to sum two numeric expressions.
///
/// This macro takes exactly two arguments, both of which should be valid Rust expressions
/// that evaluate to integer types. The macro will expand to the tuple, including value of first argument
/// repeated N times, where N is a value of second argument.
///
/// # Examples
///
/// Use the macro to sum two integers:
///
/// ```
/// use easify_macros::dynamic_tuple;
/// let result = dynamic_tuple!(2, 1);
/// assert_eq!(result, (2, ));
/// ```
///
/// ```
/// use easify_macros::dynamic_tuple;
/// let result = dynamic_tuple!(5, 3);
/// assert_eq!(result, (5, 5, 5));
/// ```
///
/// # Panics
///
/// The macro will cause a compile-time error if not exactly two arguments are provided.
///
/// ```
/// # use easify_macros::dynamic_tuple;
/// // This will fail to compile
/// // let result = sum_two!(5);
/// ```


struct ValueAndCount(Expr, LitInt);

impl Parse for ValueAndCount {
    fn parse(input: ParseStream) -> Result<Self> {
        let value = input.parse()?;
        input.parse::<Token![,]>()?;
        let count = input.parse()?;
        Ok(ValueAndCount(value, count))
    }
}

#[proc_macro]
pub fn dynamic_tuple(input: TokenStream) -> TokenStream {
    let ValueAndCount(value, count) = parse_macro_input!(input as ValueAndCount);

    // Convert value to TokenStream and then to string
    let value_ts = quote! { #value }.to_string();
    let count = count.base10_parse::<usize>().expect("Invalid count of arguments");


    // Generate the string representation of the tuple
    let tuple_elements = std::iter::repeat(value_ts)
        .take(count)
        .collect::<Vec<_>>()
        .join(", ");
    let tuple_string = format!("({})", tuple_elements);

    // Convert the string to a TokenStream and return
    tuple_string.parse().unwrap()
}