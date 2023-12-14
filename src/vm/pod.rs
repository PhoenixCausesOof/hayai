use std::fmt::Debug;

pub trait Pod: Copy + 'static + Debug {}

impl<T: Copy + 'static + Debug> Pod for T {}