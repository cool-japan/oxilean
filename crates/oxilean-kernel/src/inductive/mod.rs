//! Auto-generated module structure

pub mod derive;
pub mod focusstack_traits;
pub mod functions;
pub mod inductiveenv_traits;
pub mod inductiveerror_traits;
pub mod inductivetypebuilder_traits;
pub mod labelset_traits;
pub mod nested;
pub mod smallmap_traits;
pub mod statsummary_traits;
pub mod transformstat_traits;
pub mod types;
pub mod windowiterator_traits;

// Re-export all types
pub use derive::{
    add_inductive_family, check_and_derive_family, derive_family_unchecked, DerivedFamily,
    InductiveSpec,
};
pub use focusstack_traits::*;
pub use functions::*;
pub use inductiveenv_traits::*;
pub use inductiveerror_traits::*;
pub use inductivetypebuilder_traits::*;
pub use labelset_traits::*;
pub use nested::{
    check_and_derive_family_nested, detect as detect_nested, restore as restore_nested,
    specialize as specialize_nested, verify_nested_bundle, AuxMap, ExpandedFamily, NestedOcc,
};
pub use smallmap_traits::*;
pub use statsummary_traits::*;
pub use transformstat_traits::*;
pub use types::*;
pub use windowiterator_traits::*;
