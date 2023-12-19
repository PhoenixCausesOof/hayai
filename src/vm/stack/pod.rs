//! Simple helper trait.

use std::fmt::Debug;

/// A trait for types that can be written to the the [`Stack`]
pub trait Pod: Copy + 'static + Debug {}

impl<T: Copy + 'static + Debug> Pod for T {}