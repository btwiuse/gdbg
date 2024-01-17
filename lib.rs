#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        {
            use gstd::{debug, prelude::{file, line}};
            gstd::debug!("[{}:{}]", gstd::prelude::file!(), gstd::prelude::line!())
        }
    };
    ($val:expr $(,)?) => {
        {
            use gstd::{debug, prelude::{file, line, stringify}};
            // Use of `match` here is intentional because it affects the lifetimes
            // of temporaries - https://stackoverflow.com/a/48732525/1063961
            match $val {
                tmp => {
                    gstd::debug!("[{}:{}] {} = {:?}",
                        gstd::prelude::file!(),
                        gstd::prelude::line!(),
                        gstd::prelude::stringify!($val),
                        &tmp);
                    tmp
                }
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! dbg {
    // std implementation
    () => {
        {
            eprintln!("[{}:{}]", file!(), line!());
        }
    };
    ($val:expr $(,)?) => {
        {
            eprintln!("[{}:{}] {} = {:?}",
                file!(),
                line!(),
                stringify!($val),
                &$val);
            $val
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}
