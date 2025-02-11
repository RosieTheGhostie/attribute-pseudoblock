/// Applies the same attribute to all items in the block, but doesn't compile it as an actual block.
///
/// Code is very slightly tweaked from Todd's answer in [this Stack Overflow thread](https://stackoverflow.com/questions/58602358/applying-a-rust-attribute-to-more-than-one-line).
///
/// # Examples
///
/// Say you wanted to lock most of a file behind a certain feature "bar". With [`pseudoblock`], you
/// can do that without repeating the same attribute for practically every line of code.
///
/// ```ignore
/// # #[macro_use] extern crate attribute_pseudoblock;
/// pub mod foo;
///
/// psuedoblock! {
///     #![cfg(feature = "bar")]
///
///     mod baz;
///
///     use baz::{bee, buzz, fizz};
///
///     pub fn some_api_thing(x: i32, y: i32) -> Option<i32> {
///         // ...
/// #       None
///     }
///
///     pub fn another_api_thing(user_id: u64, password_sha256: [u8; 32]) -> bool {
///         // ...
/// #       false
///     }
/// }
/// ```
#[macro_export]
macro_rules! pseudoblock {
    { #!$attr:tt $($it:item)* } => {
        $(
            #$attr
            $it
        )*
    };
}
