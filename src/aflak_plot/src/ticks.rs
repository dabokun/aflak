use imgui::{ImString, ImVec2, Ui, WindowDrawList};

use super::AxisTransform;

const TICK_COUNT: usize = 10;

const COLOR: u32 = 0xFFFFFFFF;
const TICK_SIZE: f32 = 3.0;
const LABEL_HORIZONTAL_PADDING: f32 = 2.0;

pub struct XYTicks {
    x: XTicks,
    y: YTicks,
}

pub struct XTicks {
    labels: Vec<(ImString, ImVec2)>,
    axis_label: (ImString, ImVec2),
}

pub struct YTicks {
    labels: Vec<(ImString, ImVec2)>,
}

impl XYTicks {
    pub fn prepare<F1, F2>(
        ui: &Ui,
        xlims: (f32, f32),
        ylims: (f32, f32),
        xaxis: Option<&AxisTransform<F1>>,
        yaxis: Option<&AxisTransform<F2>>,
    ) -> Self
    where
        F1: Fn(f32) -> f32,
        F2: Fn(f32) -> f32,
    {
        Self {
            x: XTicks::prepare(ui, xlims, xaxis),
            y: YTicks::prepare(ui, ylims, yaxis),
        }
    }

    pub fn x_labels_height(&self) -> f32 {
        self.x.height()
    }

    pub fn y_labels_width(&self) -> f32 {
        self.y.width()
    }

    pub fn draw<P, S>(self, draw_list: &WindowDrawList, p: P, size: S)
    where
        P: Into<ImVec2>,
        S: Into<ImVec2>,
    {
        let p = p.into();
        let size = size.into();

        self.x.draw(draw_list, p, size);
        self.y.draw(draw_list, p, size);
    }
}

impl XTicks {
    pub fn prepare<F>(ui: &Ui, xlims: (f32, f32), axis: Option<&AxisTransform<F>>) -> Self
    where
        F: Fn(f32) -> f32,
    {
        let labels = (0..=TICK_COUNT)
            .map(|i| {
                let point = xlims.0 + i as f32 * (xlims.1 - xlims.0) / TICK_COUNT as f32;
                let label = if let Some(axis) = axis {
                    let transformed = axis.pix2world(point);
                    ImString::new(format!("{:.2}", transformed))
                } else {
                    ImString::new(format!("{:.0}", point))
                };
                let text_size = ui.calc_text_size(&label, false, -1.0);
                (label, text_size)
            }).collect();
        let axis_label = {
            let unit = axis.map(|axis| axis.unit()).unwrap_or("");
            let label = ImString::new(unit);
            let text_size = ui.calc_text_size(&label, false, -1.0);
            (label, text_size)
        };
        XTicks { labels, axis_label }
    }

    pub fn height(&self) -> f32 {
        self.labels
            .iter()
            .fold(0.0, |acc, (_, size)| acc.max(size.y))
    }

    pub fn draw<P, S>(self, draw_list: &WindowDrawList, p: P, size: S)
    where
        P: Into<ImVec2>,
        S: Into<ImVec2>,
    {
        let p = p.into();
        let size = size.into();

        let x_step = size.x / (self.labels.len() - 1) as f32;
        let mut x_pos = p.x;
        let y_pos = p.y + size.y;
        let mut label_height = 0.0f32;
        for (label, text_size) in self.labels {
            draw_list
                .add_line([x_pos, y_pos], [x_pos, y_pos - TICK_SIZE], COLOR)
                .build();
            draw_list.add_text([x_pos - text_size.x / 2.0, y_pos], COLOR, label.to_str());
            x_pos += x_step;
            label_height = label_height.max(text_size.y);
        }

        let (label, text_size) = self.axis_label;
        let middle_x = p.x + size.x / 2.0;
        draw_list.add_text(
            [middle_x - text_size.x / 2.0, y_pos + label_height],
            COLOR,
            label,
        );
    }
}

impl YTicks {
    pub fn prepare<F>(ui: &Ui, ylims: (f32, f32), axis: Option<&AxisTransform<F>>) -> Self
    where
        F: Fn(f32) -> f32,
    {
        let labels = (0..=TICK_COUNT)
            .map(|i| {
                let point = ylims.0 + i as f32 * (ylims.1 - ylims.0) / TICK_COUNT as f32;
                let label = if let Some(axis) = axis {
                    let transformed = axis.pix2world(point);
                    ImString::new(format!("{:.2}", transformed))
                } else {
                    ImString::new(format!("{:.0}", point))
                };
                let text_size = ui.calc_text_size(&label, false, -1.0);
                (label, text_size)
            }).collect();
        YTicks { labels }
    }

    pub fn width(&self) -> f32 {
        self.labels
            .iter()
            .fold(0.0, |acc, (_, size)| acc.max(size.x))
    }

    pub fn draw<P, S>(self, draw_list: &WindowDrawList, p: P, size: S)
    where
        P: Into<ImVec2>,
        S: Into<ImVec2>,
    {
        let p = p.into();
        let size = size.into();

        let y_step = size.y / (self.labels.len() - 1) as f32;
        let mut y_pos = p.y + size.y;
        let x_pos = p.x;
        for (label, text_size) in self.labels {
            draw_list
                .add_line([x_pos, y_pos], [x_pos + TICK_SIZE, y_pos], COLOR)
                .build();
            draw_list.add_text(
                [
                    x_pos - text_size.x - LABEL_HORIZONTAL_PADDING,
                    y_pos - text_size.y / 2.0,
                ],
                COLOR,
                label.to_str(),
            );
            y_pos -= y_step;
        }
    }
}

pub fn add_ticks<P, S, F1, F2>(
    ui: &Ui,
    draw_list: &WindowDrawList,
    p: P,
    size: S,
    xlims: (f32, f32),
    ylims: (f32, f32),
    xaxis: Option<&AxisTransform<F1>>,
    yaxis: Option<&AxisTransform<F1>>,
) where
    P: Into<ImVec2>,
    S: Into<ImVec2>,
    F1: Fn(f32) -> f32,
    F2: Fn(f32) -> f32,
{
    XYTicks::prepare(ui, xlims, ylims, xaxis, yaxis).draw(draw_list, p, size)
}
