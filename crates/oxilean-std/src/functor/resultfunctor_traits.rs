//! # ResultFunctor - Trait Implementations
//!
//! This module contains trait implementations for `ResultFunctor`.
//!
//! ## Implemented Traits
//!
//! - `Functor`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Functor;
use super::types::ResultFunctor;

impl<A, E> Functor<A> for ResultFunctor<A, E> {
    type Mapped<B> = ResultFunctor<B, E>;
    fn fmap<B, F: Fn(A) -> B>(self, f: F) -> ResultFunctor<B, E> {
        ResultFunctor(self.0.map(f))
    }
}
