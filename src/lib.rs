//! A handy library for measuring the execution time of function.
//!
//! # Use
//! The use of the crate is through the 4 attribute macros:
//! `#[timer_println]`, `#[timer_log_trace]`, `#[timer_log_info]`, `#[timer_log_debug]`.
//!
//! Just like the name they have, `timer_println` means that it will be using the macro `println!` to
//! print the results, while `timer_log_trace`, `timer_log_info` and `timer_log_debug` respectively
//! using macros `log::trace!`, `log::info!` and `log::debug!` to output.
//!
//! Of course, when using the last 3 macros, you should link the log facade and its implementation.
//!
//! ## Examples
//! ```toml
//! [dependencies]
//! calculagraph = "0.1"
//! ```
//! ```rust
//! use std::{thread, time};
//! use calculagraph::timer_println;
//!
//! #[timer_println(ms)]
//! fn main() {
//!     thread::sleep(time::Duration::from_millis(10));
//!     println!("job done");
//! }
//! ```
//! The above example will print `fn:main cost 10ms` at the end, You can also use the second
//! argument to define the format string you need.
//!
//! ## Note
//! This macro added two variables that would never conflict in ordinary business, they are
//! `now_x7bf707c839bc2554fa3f1913a8dc699b68236726c5da18b31f660948ca7f542a267de9b` and
//! `result_x7bf707c839bc2554fa3f1913a8dc699b68236726c5da18b31f660948ca7f542a267de9b`, just don't
//! use them intentionally.
//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, AttributeArgs, Block, Error, Item, ItemFn, Lit,
    Meta, NestedMeta, Path, Result, Signature, Visibility,
};

// A suffix with a very low probability of conflict,
// You should not use `now_$UID_SUFFIX$` or `result_$UID_SUFFIX$` as variable name in functions that use the macro.
// This will cause unexpected result.
const UID_SUFFIX: &str = "x7bf707c839bc2554fa3f1913a8dc699b68236726c5da18b31f660948ca7f542a267de9b";

/// `std::println!` the execution time after the function is called and executed.
///
/// The macro support none, 1 or 2 parameters, [(`TimeUnit`[, `FormatString`])].
/// The parameter `TimeUnit` supports four types of `s`, `ms`(by default), `us` and `ns`,
/// The parameter `FormatString` is similar to the format string parameter in `println!`, note that
/// only one placeholder is supported here, which will fill the time result.
/// ### Examples
/// ```
/// #[timer_println]
/// fn func() {}
///
/// #[timer_println(ns)]
/// fn func1() {}
///
/// #[timer_println(ns, "func2() execution time: {}ns")]
/// fn func2() {}
/// ```
///
#[proc_macro_attribute]
pub fn timer_println(attr: TokenStream, input: TokenStream) -> TokenStream {
    builder(
        &parse_macro_input!(attr as AttributeArgs),
        &parse_macro_input!(input as Item),
        &quote!( println! ),
    )
    .unwrap()
}

/// `log::info!` the execution time after the function is called and executed,
///
/// The macro support none, 1 or 2 parameters, [(`TimeUnit`[, `FormatString`])].
/// The parameter `TimeUnit` supports four types of `s`, `ms`(by default), `us` and `ns`,
/// The parameter `FormatString` is similar to the format string parameter in `println!`, note that
/// only one placeholder is supported here, which will fill the time result.
/// ### Examples
/// ```
/// #[timer_log_info]
/// fn func() {}
///
/// #[timer_log_info(ns)]
/// fn func1() {}
///
/// #[timer_log_info(ns, "func2() execution time: {}ns")]
/// fn func2() {}
/// ```
///
#[proc_macro_attribute]
pub fn timer_log_info(attr: TokenStream, input: TokenStream) -> TokenStream {
    builder(
        &parse_macro_input!(attr as AttributeArgs),
        &parse_macro_input!(input as Item),
        &quote!( log::info! ),
    )
    .unwrap()
}

/// `log::debug!` the execution time after the function is called and executed,
///
/// The macro support none, 1 or 2 parameters, [(`TimeUnit`[, `FormatString`])].
/// The parameter `TimeUnit` supports four types of `s`, `ms`(by default), `us` and `ns`,
/// The parameter `FormatString` is similar to the format string parameter in `println!`, note that
/// only one placeholder is supported here, which will fill the time result.
/// ### Examples
/// ```
/// #[timer_log_debug]
/// fn func() {}
///
/// #[timer_log_debug(ns)]
/// fn func1() {}
///
/// #[timer_log_debug(ns, "func2() execution time: {}ns")]
/// fn func2() {}
/// ```
///
#[proc_macro_attribute]
pub fn timer_log_debug(attr: TokenStream, input: TokenStream) -> TokenStream {
    builder(
        &parse_macro_input!(attr as AttributeArgs),
        &parse_macro_input!(input as Item),
        &quote!( log::debug! ),
    )
    .unwrap()
}

/// `log::trace!` the execution time after the function is called and executed,
///
/// The macro support none, 1 or 2 parameters, [(`TimeUnit`[, `FormatString`])].
/// The parameter `TimeUnit` supports four types of `s`, `ms`(by default), `us` and `ns`,
/// The parameter `FormatString` is similar to the format string parameter in `println!`, note that
/// only one placeholder is supported here, which will fill the time result.
/// ### Examples
/// ```
/// #[timer_log_trace]
/// fn func() {}
///
/// #[timer_log_trace(ns)]
/// fn func1() {}
///
/// #[timer_log_trace(ns, "func2() execution time: {}ns")]
/// fn func2() {}
/// ```
///
#[proc_macro_attribute]
pub fn timer_log_trace(attr: TokenStream, input: TokenStream) -> TokenStream {
    builder(
        &parse_macro_input!(attr as AttributeArgs),
        &parse_macro_input!(input as Item),
        &quote!( log::trace! ),
    )
    .unwrap()
}

#[derive(Debug, Copy, Clone)]
enum TimeUnit {
    S,
    MS,
    US,
    NS,
}

impl TimeUnit {
    fn quote(self) -> TokenStream2 {
        match self {
            TimeUnit::S => quote!(as_secs()),
            TimeUnit::MS => quote!(as_millis()),
            TimeUnit::US => quote!(as_micros()),
            TimeUnit::NS => quote!(as_nanos()),
        }
    }
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TimeUnit::S => write!(f, "s"),
            TimeUnit::MS => write!(f, "ms"),
            TimeUnit::US => write!(f, "us"),
            TimeUnit::NS => write!(f, "ns"),
        }
    }
}

impl std::convert::From<String> for TimeUnit {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "S" => TimeUnit::S,
            "MS" => TimeUnit::MS,
            "US" => TimeUnit::US,
            "NS" => TimeUnit::NS,
            _ => panic!("Invalid unit of time, only `s`, `ms`, `us`, `ns` are supported"),
        }
    }
}

fn builder(args: &AttributeArgs, body: &Item, outputter: &TokenStream2) -> Result<TokenStream> {
    let (f_attrs, f_vis, f_sig, f_block) = parse_body(body)?;
    let f_name = &f_sig.ident.to_string();
    let (t_unit, o_format) = parse_args(args, f_name)?;

    return Ok((move || {
        let t_formatter = t_unit.quote();
        let v_now = format_ident!("now_{}", UID_SUFFIX);
        let v_result = format_ident!("result_{}", UID_SUFFIX);
        let f_stmts = &f_block.stmts;
        quote!(
            #(#f_attrs)* #f_vis #f_sig {
                let #v_now = std::time::Instant::now();
                let #v_result = (move ||{ #(#f_stmts)* })();
                #outputter(#o_format, #v_now . elapsed(). #t_formatter );
                return #v_result;
            }
        )
    })()
    .into());

    // -> (time_unit, output_format_string)
    fn parse_args(args: &AttributeArgs, fn_name: &String) -> Result<(TimeUnit, String)> {
        match &args[..] {
            [] => Ok((TimeUnit::MS, format!("fn:{} cost {{}}ms", fn_name))),
            [u] => {
                let u = read_time_unit(u)?;
                Ok((u, format!("fn:{} cost {{}}{}", fn_name, u)))
            }
            [u, f] => Ok((read_time_unit(u)?, read_output_format(f)?)),
            _ => panic!("Invalid arguments, usage: [(TimeUnit[, OutputFormatString])]"),
        }
    }

    fn parse_body(body: &Item) -> Result<(&Vec<Attribute>, &Visibility, &Signature, &Box<Block>)> {
        match body {
            Item::Fn(f @ _) => {
                let ItemFn {
                    attrs,
                    vis,
                    sig,
                    block,
                } = f;
                Ok((attrs, vis, sig, block))
            }
            _ => Err(Error::new(
                body.span(),
                "Statement other than function are not supported",
            )),
        }
    }

    fn read_time_unit(u: &NestedMeta) -> Result<TimeUnit> {
        if let NestedMeta::Meta(Meta::Path(Path { segments, .. })) = u {
            return Ok(segments[0].ident.to_string().into());
        }
        Err(syn::Error::new(
            u.span(),
            "Invalid argument, `TimeUnit` expected",
        ))
    }

    fn read_output_format(f: &NestedMeta) -> Result<String> {
        match f {
            NestedMeta::Lit(Lit::Str(s)) => Ok(s.value()),
            _ => Err(syn::Error::new(
                f.span(),
                "Invalid FormatString, the usage is similar with macro `println!` or `format!`",
            )),
        }
    }
}
