//! # VecFunctor - Trait Implementations
//!
//! This module contains trait implementations for `VecFunctor`.
//!
//! ## Implemented Traits
//!
//! - `Functor`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Functor;
use super::types::VecFunctor;

impl<A> Functor<A> for VecFunctor<A> {
    type Mapped<B> = VecFunctor<B>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> VecFunctor<B> {
        VecFunctor(self.0.into_iter().map(f).collect())
    }
}
