mod state;

use imgui::Ui;
use ndarray::Array1;

use super::interactions;
use super::lims;
use super::ticks;
use super::util;
use super::AxisTransform;
use super::Error;

pub use self::state::State;

pub trait UiImage1d {
    fn image1d<F>(
        &self,
        image: &Array1<f32>,
        vunit: &str,
        axis: Option<AxisTransform<F>>,
        state: &mut State,
    ) -> Result<(), Error>
    where
        F: Fn(f32) -> f32;
}

impl<'ui> UiImage1d for Ui<'ui> {
    fn image1d<F>(
        &self,
        image: &Array1<f32>,
        vunit: &str,
        axis: Option<AxisTransform<F>>,
        state: &mut State,
    ) -> Result<(), Error>
    where
        F: Fn(f32) -> f32,
    {
        let p = self.get_cursor_screen_pos();
        let window_pos = self.get_window_pos();
        let window_size = self.get_window_size();
        let size = (window_size.0, window_size.1 - (p.1 - window_pos.1));
        state.plot(self, image, vunit, axis, p, size)
    }
}
