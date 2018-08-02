#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

#[macro_use]
mod layout;

mod color;
mod context;
mod event;
mod events;
mod font;
mod geometry;
mod node;
mod os;
mod text;
mod view;
mod window;

// TODO: Submit documentation for these types to `yoga`.
pub use yoga::{Align, Edge, FlexDirection, Justify, PositionType, Wrap};

pub use self::{
    color::*, context::*, event::*, events::*, font::*, geometry::*, layout::*, node::*, text::*,
    view::*, window::*,
};
