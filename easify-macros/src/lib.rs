use proc_macro::TokenStream;
// someting goes here
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, Expr, ExprLet, Ident, LitInt, LitStr,
    Local, Result, Token,
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
/// The macro will cause a compile-time error if not exactly two arguments are provided.
///
/// ```
/// # use easify_macros::dynamic_tuple;
/// // This will fail to compile
/// // let result = sum_two!(5);
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

    // Convert the string to a TokenStream and return
    tuple_string.parse().unwrap()
}

/// A procedural macro to split the string and return tuple, where you know the number of
/// arguments, thus making it easy to unpack.
///
/// This macro takes exactly two arguments, both of which should be valid Rust expressions
/// that evaluate to str types. The macro will expand to the tuple, including value of first argument
/// repeated N times, where N is a value of second argument.
///
/// # Examples
///
/// Use the macro to split the string:
///
/// ```
/// use easify_macros::dynamic_tuple;
/// let result = unpack_split!("hello,my,name", ",");
/// assert_eq!(result, ("hello", "my", "name"));
/// ```
///
/// ```
/// use easify_macros::dynamic_tuple;
/// let some_text = "hello,world";
/// let result = unpack_split!(some_text, 3);
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

// Define a struct to hold the parsed elements
struct LetStatement {
    vars: Vec<Ident>,
    astrix: usize,
    expr: Expr,
}

// Implement custom parsing for the LetStatement
impl Parse for LetStatement {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![let]>()?; // Parse the 'let' keyword

        let mut vars = Vec::new();
        let mut count: usize = 0;
        let mut astrix: usize = 0;
        // Parse variables
        loop {
            if input.peek(Token![*]) {
                input.parse::<Token![*]>()?; // Parse the '*' if present
                astrix = count;
            }
            let var: Ident = input.parse()?;
            vars.push(var);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
            count += 1;
        }

        input.parse::<Token![=]>()?; // Parse the '='

        // Parse the right-hand side expression
        let expr: Expr = input.parse()?;

        Ok(LetStatement { vars, astrix, expr })
    }
}

// Procedural macro
#[proc_macro]
pub fn smart_split(input: TokenStream) -> TokenStream {
    let LetStatement { vars, astrix, expr } = parse_macro_input!(input as LetStatement);
    match &expr{
        Expr::Path(path) => {
            quote! {
                // Here `expr` is used directly assuming it's a Vec<T>
                for element in #expr.iter() {
                    println!("{:?}", element);
                }
            }
        },
        Expr::Lit(expr_lit) => {
            quote! {
                eprintln!("{:?}", expr_lit);
            }
        },
        _ => quote! {
            eprintln!("Invalid expression"),
        }
    };
    eprintln!("{:?}", astrix);
    // Create a string with the variable names
    let vars_string = vars
        .iter()
        .map(Ident::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    TokenStream::from(quote! {
    })
}
