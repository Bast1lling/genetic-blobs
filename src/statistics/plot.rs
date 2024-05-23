use crate::{Model, Nannou};
use nannou::geom::{pt2, Rect};
use nannou::{color, Draw};

pub struct Plot {
    nannou_frame: Rect<f32>,
    data: Vec<f32>,
}

impl Nannou for Plot {
    fn draw(&self, draw: &Draw, _model: &Model) {
        let frame = self.nannou_frame;

        // background of plot
        draw.rect()
            .xy(frame.xy())
            .wh(frame.wh())
            .color(color::WHITE);

        // axis
        let ends = [
            frame.mid_right(),
            frame.mid_top(),
            frame.mid_left(),
            frame.mid_bottom(),
        ];

        for &end in &ends {
            draw.arrow()
                .start(frame.xy())
                .start_cap_round()
                .head_length(4.0)
                .head_width(2.0)
                .end(end)
                .color(color::BLACK);
        }

        // function
        let values = &self.data;
        let x_start: f32 = frame.left();
        let step: f32 = frame.w() / values.len() as f32;

        let points = (0..values.len()).map(|i| {
            let y = values[i];
            let x = x_start + i as f32 * step;
            (pt2(x, y), color::RED)
        });

        draw.polyline().weight(2.0).points_colored(points);
    }

    fn update(&mut self) {}
}
