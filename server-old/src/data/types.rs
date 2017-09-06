use std::fmt;
use std::slice::Iter;

/// A helper macro to get the length of expressions in a macro
/// TODO: Remove this, if it's implemented in Rust. Right now they plan to
/// to make this a post v1.0 feature.
macro_rules! count_expr {
    () => { 0 };
    ($_e: expr $(, $rest: expr)*) => { 1 + count_expr!($($rest),*) }
}

/// Use this macro to generate rust enums which implement:
///
/// - The `fmt::Display` trait
/// - The function `from_str` which gets the correct Type from the enum by it's
///    name from a `str`.
/// - The function `from_string` which gets the correct Type from the enum by it's
///    name from a `String`.
/// - An Iterator to iterate over every Type as string. `::iterator()`
///
/// ```
/// pub fn from_str(name: &str) -> Result<$enumname, ()> {
///     match name {
///         $(
///             stringify!($enumvals) => Ok($enumname::$enumvals),
///         )*
///         _ => Err(()),
///     }
/// }
/// ```
#[macro_export]
macro_rules! enum_impl {
    ($enumname: ident {
        $($enumvals: ident,)*
    }) => {
        #[derive(Debug, Eq, Clone, Hash, PartialEq, Serialize, Deserialize)]
        pub enum $enumname {
            $($enumvals,)*
        }

        impl fmt::Display for $enumname {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }

        impl $enumname {
            #[allow(dead_code)]
            pub fn from_str(name: &str) -> Result<$enumname, ()> {
                match name {
                    $(
                        stringify!($enumvals) => Ok($enumname::$enumvals),
                    )*
                    _ => Err(()),
                }
            }
            #[allow(dead_code)]
            pub fn from_string(string_name: &String) -> Result<$enumname, ()> {
                let name = string_name.as_str();
                match name {
                    $(
                        stringify!($enumvals) => Ok($enumname::$enumvals),
                    )*
                    _ => Err(()),
                }
            }
            #[allow(dead_code)]
            pub fn iterator() -> Iter<'static, $enumname> {
                static ITERATOR: [$enumname; count_expr!($($enumvals),+)] = [
                    $(
                        $enumname::$enumvals,
                    )*
                ];
                ITERATOR.into_iter()
            }
        }
    }
}

/// This enum contains all types of valid researches.
/// It's used to check against the names in `research_data.yml`, to validate types in requests
/// and to guarantee database string integrity.
enum_impl!{ResearchTypes {
    Plasma,
    EnergyWeapons,
    MiningEfficiency,
}}

/// This enum contains all types of valid resources.
/// It's used to check against the costs in `research_data.yml` and `module_data.yml`,
/// to validate types in requests and to guarantee database string integrity.
enum_impl!{ResourceTypes {
    Iron,
    Water,
}}

/// This enum contains all types of valid modules.
/// It's used to check against the names in `module_data.yml`, to validate types in requests
/// and to guarantee database string integrity.
enum_impl!{ModuleTypes {
    LaserTurret,
    PlasmaGenerator,
}}