//! # OptionFunctor - Trait Implementations
//!
//! This module contains trait implementations for `OptionFunctor`.
//!
//! ## Implemented Traits
//!
//! - `Functor`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Functor;
use super::types::OptionFunctor;

impl<A> Functor<A> for OptionFunctor<A> {
    type Mapped<B> = OptionFunctor<B>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> OptionFunctor<B> {
        OptionFunctor(self.0.map(f))
    }
}
