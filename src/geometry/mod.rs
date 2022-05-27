mod rectangle;
mod triangle;
mod point;

pub trait Area {
    fn area(&self) -> f32;
}

pub(crate) use point::*;
pub(crate) use rectangle::*;
pub(crate) use triangle::*;
