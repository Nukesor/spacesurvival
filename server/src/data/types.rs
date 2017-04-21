use std::fmt;

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

        }
    }
}

enum_impl!{ResearchTypes {
    PlasmaGenerator,
    EnergyWeapons,
    MiningEfficiency,
}}

enum_impl!{ResourceTypes {
    Iron,
    Water,
}}

enum_impl!{ModuleTypes {
    Turret,
    Generator,
}}
