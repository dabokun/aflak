//! Plotting library for aflak.
//!
//! Please see the examples in the repository of this crate to get an idea of
//! how it is used.
//!
//! Basically, this crate defines and implements two traits on imgui's `Ui`
//! objet. These are [UiImage1d](plot/trait.UiImage1d.html) and
//! [UiImage2d](imshow/trait.UiImage2d.html).
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
#[macro_use]
extern crate ndarray;

pub mod imshow;
pub mod plot;

mod err;
mod interactions;
mod lims;
mod ticks;
mod units;
mod util;

pub use err::Error;
pub use interactions::{Interaction, InteractionId, InteractionIterMut, Value, ValueIter};
pub use units::AxisTransform;
