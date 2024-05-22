use crate::evolution::gene::Genome;
use crate::{Model, Nannou};
use nannou::color::Rgb;
use nannou::geom::{pt2, Point2, Vec2};
use nannou::Draw;
/// external crate
use rand::{thread_rng, Rng};

use super::gene::Inform;

pub const SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn as_color(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl Inform for RGB {
    fn rate(&self) -> f32 {
        // let weight = 1.0/(3.0 * SIZE as f32 * SIZE as f32);
        let treshold: u8 = 15;
        if self.r < treshold && self.g < treshold && self.b < treshold {
            1.0
        } else {
            0.0
        }
    }

    fn random(_example: Option<Self>) -> Self {
        let mut rng = thread_rng();
        RGB {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blob {
    pub genome: Genome<RGB>,
    nannou_size: f32,
    nannou_position: Point2,
}

impl Nannou for Blob {
    fn draw(&self, draw: &Draw, model: &Model) {
        let position = model.transform(self.nannou_position);
        let size = self.nannou_size * model.zoom;
        let mode = 0;

        match mode {
            0 => self.draw_rect(draw, position, size),
            1 => self.draw_simple_rect(draw, position, size),
            2 => self.draw_circle(draw),
            3 => self.draw_simple_circle(draw),
            _ => panic!("This drawing mode does not exist!"),
        }
    }

    fn update(&mut self) {}
}

impl Blob {

    pub fn new(genome: Genome<RGB>, nannou_size: f32, nannou_position: Point2) -> Self
    {
        Self {
            genome: genome,
            nannou_size,
            nannou_position,
        }
    }

    pub fn draw_rect(&self, draw: &Draw, at: Vec2, size: f32) {
        let offset = (SIZE as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);
        (0..(SIZE * SIZE)).for_each(|i| {
            let temp = i / SIZE;
            let y = bottom_left.1 + temp as f32 * size;
            let x = bottom_left.0 + (i % SIZE) as f32 * size;
            draw.rect()
                .x_y(x, y)
                .w_h(size, size)
                .color(Rgb::from_components(self.genome.data[i].as_color()));
        });
    }

    pub fn draw_simple_rect(&self, draw: &Draw, at: Vec2, size: f32) {
        let offset = (SIZE as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);

        (0..SIZE).for_each(|row| {
            let points = (0..SIZE).map(|column| {
                let index = row * SIZE + column;
                let x = bottom_left.0 + column as f32 * size;
                let y = bottom_left.1 + row as f32 * size;
                (pt2(x, y), Rgb::from_components(self.genome.data[index].as_color()))
            });
            draw.polyline().weight(size).points_colored(points);
        })
    }

    pub fn draw_circle(&self, draw: &Draw) {
        let offset = (SIZE as f32 / 2.) * self.nannou_size - self.nannou_size / 2.;
        let bottom_left = (
            self.nannou_position.x - offset,
            self.nannou_position.y - offset,
        );
        (0..(SIZE * SIZE)).for_each(|i| {
            let temp = i / SIZE;
            let y = bottom_left.1 + temp as f32 * self.nannou_size;
            let x = bottom_left.0 + (i % SIZE) as f32 * self.nannou_size;
            let dist = self.nannou_position.distance(Vec2::new(x, y));

            if dist <= offset {
                draw.rect()
                    .x_y(x, y)
                    .w_h(self.nannou_size, self.nannou_size)
                    .color(Rgb::from_components(self.genome.data[i].as_color()));
            }
        });
    }

    pub fn draw_simple_circle(&self, draw: &Draw) {
        let offset = (SIZE as f32 / 2.) * self.nannou_size - self.nannou_size / 2.;
        let bottom_left = (
            self.nannou_position.x - offset,
            self.nannou_position.y - offset,
        );

        (0..SIZE).for_each(|row| {
            let mut points = Vec::new();
            for column in 0..SIZE {
                let index = row * SIZE + column;
                let x = bottom_left.0 + column as f32 * self.nannou_size;
                let y = bottom_left.1 + row as f32 * self.nannou_size;
                let dist = self.nannou_position.distance(Vec2::new(x, y));

                if dist <= offset {
                    points.push((pt2(x, y), Rgb::from_components(self.genome.data[index].as_color())))
                }
            }
            draw.polyline()
                .weight(self.nannou_size)
                .points_colored(points);
        })
    }
}
