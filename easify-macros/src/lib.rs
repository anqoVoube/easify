use proc_macro::TokenStream;
// someting goes here
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, Expr, Ident, LitInt, LitStr,
    Result, Token,
};

struct DynamicTupleParams(Expr, LitInt);
struct SmartSplitParams(Ident, LitStr, LitInt);

impl Parse for DynamicTupleParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let value = input.parse()?;
        input.parse::<Token![,]>()?;
        let count = input.parse()?;
        Ok(DynamicTupleParams(value, count))
    }
}

impl Parse for SmartSplitParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let text = input.parse()?;
        input.parse::<Token![,]>()?;
        let sep = input.parse()?;
        input.parse::<Token![,]>()?;
        let count = input.parse()?;
        Ok(SmartSplitParams(text, sep, count))
    }
}
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
/// The macro will cause arguments_number compile-time error if not exactly two arguments are provided.
///
/// ```
/// # use easify_macros::dynamic_tuple;
/// // This will fail to compile
/// // let result = dynamic_tuple!(5);
/// ```

#[proc_macro]
pub fn dynamic_tuple(input: TokenStream) -> TokenStream {
    let DynamicTupleParams(value, count) = parse_macro_input!(input as DynamicTupleParams);

    // Convert value to TokenStream and then to string
    let value_ts = quote! { #value }.to_string();
    let count = count
        .base10_parse::<usize>()
        .expect("Invalid count of arguments");

    // Generate the string representation of the tuple
    let tuple_elements = std::iter::repeat(value_ts)
        .take(count)
        .collect::<Vec<_>>()
        .join(", ");
    let tuple_string = format!("({})", tuple_elements);

    // Convert the string to arguments_number TokenStream and return
    tuple_string.parse().unwrap()
}

/// A procedural macro to split the string and return tuple, where you know the number of
/// arguments, thus making it easy to unpack.
///
/// This macro takes exactly two arguments, both of which should be valid Rust expressions
/// that evaluate to str types. The macro will expand to the tuple, including value of first argument
/// repeated N times, where N is arguments_number value of second argument.
///
/// # Examples
///
/// Use the macro to split the string:
///
/// ```
/// use easify_macros::unpack_split;
/// let result = unpack_split!("hello,my,name", ",");
/// assert_eq!(result, ("hello", "my", "name"));
/// ```
///
/// ```
/// use easify_macros::unpack_split;
/// let some_text = "hello,world";
/// let result = unpack_split!(some_text, 2);
/// assert_eq!(result, ("hello", "world"));
/// ```
///
/// # Panics
///
/// The macro will cause arguments_number compile-time error if not exactly two arguments are provided.
///
/// ```
/// # use easify_macros::unpack_split;
/// // This will fail to compile
/// // let result = unpack_split!("hello,world");
/// ```

#[proc_macro]
pub fn unpack_split(input: TokenStream) -> TokenStream {
    let SmartSplitParams(text, sep, count) = parse_macro_input!(input as SmartSplitParams);
    let len = count.base10_parse::<usize>().unwrap();

    let indices = 0..len;
    let tuple_elems = indices.map(|_| quote! {tuple_elements.next().unwrap()});
    let output = quote! {
        {

            let mut tuple_elements = #text.split(#sep);
            (#( #tuple_elems, )*)
        }
    };
    output.into()
}

// Define arguments_number struct to hold the parsed elements
struct LetStatement {
    vars: Vec<proc_macro2::TokenStream>,
    astrix: usize,
    expr: Ident,
}

// Implement custom parsing for the LetStatement
impl Parse for LetStatement {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut vars = Vec::new();

        let mut count: usize = 0;
        let mut astrix: usize = 0;
        // Parse variables
        loop {
            if input.peek(Token![*]) {
                input.parse::<Token![*]>()?; // Parse the '*' if present
                astrix = count;

            }

            let is_mut = input.peek(Token![mut]);
            if is_mut {
                input.parse::<Token![mut]>()?; // Parse the '*' if present
            }
            let var: Ident = input.parse()?;

            if is_mut {
                vars.push(quote!(mut #var));
            } else {
                vars.push(quote!(#var));

            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
            count += 1;
        }

        input.parse::<Token![=]>()?; // Parse the '='

        // Parse the right-hand side expression
        let expr: Ident = input.parse()?;

        Ok(LetStatement { vars, astrix, expr })
    }
}

/// A procedural macro to unpack like in Python programming language, where you expect array of
/// known size, thus making it easy to unpack.
///
/// This macro consist of specifying initial variables that your given array will be unpacked to.
/// You can also make your variable mutable, by simply adding `mut` before the variable name.
/// The astrix `*` is used to unpack the rest of the array, resulting new array.
/// Note, that you can use * only once in any position. The macro will expand to the
/// let statements and named to variables you have specified.
/// This macro can result to unexpected behavior and panicks if you don't know the Python unpacking
/// Use it with caution.
/// And remember, with great power comes great responsibility.
/// # Examples
///
///
///
/// Use the macro to split the string:
///
/// ```
/// use easify_macros::let_unpack;
/// let unpacking = vec![1, 2, 3];
/// let result = let_unpack!(*mut a, b, c = unpacking);
/// a.push(10);
/// assert_eq!(a, vec![1, 10]);
/// assert_eq!(b, 2);
/// assert_eq!(c, 3);
/// ```
///
/// ```
/// use easify_macros::let_unpack;
/// let unpacking = vec![2, 3]
/// let result = let_unpack!(a, *b, c = unpacking);
/// a.push(10);
/// assert_eq!(a, 2);
/// assert_eq!(b, vec![]);
/// assert_eq!(c, 3);
/// ```
///
/// # Panics
///
/// The macro will cause arguments_number compile-time error if not exactly two arguments are provided.
///
/// ```
/// # use easify_macros::let_unpack;
/// // This will fail to compile
/// // let result = let_unpack!("hello,world");
/// ```
// Procedural macro

#[proc_macro]
pub fn let_unpack(input: TokenStream) -> TokenStream {
    let LetStatement { vars, astrix, expr} = parse_macro_input!(input as LetStatement);
    let mut values = Vec::new();
    let arguments_number = vars.len();
    for index in 0..arguments_number {
        if index < astrix {
            values.push(quote! {#expr[#index]});
        } else if index == astrix {
            values.push(
                quote! {&#expr[#index..#expr.len() - #arguments_number + #index + 1].to_vec()}
            );
        } else {
            values.push(quote! {#expr[#expr.len() - #arguments_number + #index]});
        }
    }
    TokenStream::from(quote! {
        #( let #vars = #values; )*;
        drop(#expr);
    })

}
