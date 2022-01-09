#![feature(once_cell)]

#[macro_use]
extern crate lazy_static;

use std::fmt::{Debug, Display};
use std::ops::Range;

pub mod app;
pub mod cli;
mod config;
mod event;
mod graph;
mod host;
mod net;
mod processor;
mod sim;
mod task;
mod time;
mod units;
mod worker;

#[derive(Debug)]
pub struct OutOfBoundsError<T>
where
    T: Copy + Debug + Display,
{
    bounds: Range<T>,
    actual: T,
}

impl<T> OutOfBoundsError<T>
where
    T: Copy + Debug + Display,
{
    fn new(lower: T, upper: T, actual: T) -> Self {
        let bounds = lower..upper;
        Self { bounds, actual }
    }
}

impl<T> Display for OutOfBoundsError<T>
where
    T: Copy + Debug + Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.bounds.start;
        let end = self.bounds.end;
        write!(
            formatter,
            "value `{}` not within bounds `{}..{}`",
            self.actual, start, end
        )
    }
}

impl<T> std::error::Error for OutOfBoundsError<T> where T: Copy + Debug + Display {}
