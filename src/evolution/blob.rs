use nannou::color::Rgb;
use nannou::geom::{pt2, Point2, Vec2};
use nannou::Draw;
/// external crate
use rand::{thread_rng, Rng};

use crate::{
    Model, Nannou,
    util::Create,
    evolution::{
        gene::{Creature, Genome, Compare},
        square::{Square, Quadrant},
    }};

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn as_color(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl Create for RGB {
    type Params = ();

    fn create() -> Self {
        let mut rng = thread_rng();
        RGB {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
        }
    }

    fn create_like(_params: Option<Self::Params>) -> Self {
        Self::create()
    }
}

impl Compare for RGB {
    fn compare(&self, to: &Self) -> f32 {
        let norm = (3 * 255) as f32;
        let r_diff = (self.r as f32 - to.r as f32).abs();
        let g_diff = (self.g as f32 - to.g as f32).abs();
        let b_diff = (self.b as f32 - to.b as f32).abs();
        (norm - (r_diff + g_diff + b_diff))/norm
    }
}

#[derive(Debug, Clone)]
pub struct Blob {
    pub genome: Genome<RGB>,
    nannou_size: f32,
    nannou_position: Point2,
    pub velocity: Vec2,
}

impl Creature<RGB> for Blob {
    fn extract_genome(&mut self) -> &mut Genome<RGB> {
        &mut self.genome
    }
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
            4 => self.draw_debug(draw, position, size),
            _ => panic!("This drawing mode does not exist!"),
        }
    }

    fn update(&mut self) {
        self.nannou_position += self.velocity;
    }
}

impl Blob {
    pub fn new(genome: Genome<RGB>, nannou_size: f32, nannou_position: Point2) -> Self {
        Self {
            genome: genome,
            nannou_size,
            nannou_position,
            velocity: Vec2::ZERO,
        }
    }

    pub fn draw_rect(&self, draw: &Draw, at: Vec2, size: f32) {
        let width = (self.genome.len() as f32).sqrt() as usize;
        let offset = (width as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);
        (0..self.genome.len()).for_each(|i| {
            let temp = i / width;
            let y = bottom_left.1 + (temp as f32) * size;
            let x = bottom_left.0 + (i % width as usize) as f32 * size;
            draw.rect()
                .x_y(x, y)
                .w_h(size, size)
                .color(Rgb::from_components(self.genome[i].as_color()));
        });
    }

    pub fn draw_debug(&self, draw: &Draw, at: Vec2, size: f32) {
        let width = (self.genome.len() as f32).sqrt() as usize;
        let offset = (width as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);
        let quadrant = self.genome.get_quadrant(Quadrant::TopTriangularQuadrant);
        (0..self.genome.len()).for_each(|i| {
            let temp = i / width;
            let y = bottom_left.1 + (temp as f32) * size;
            let x = bottom_left.0 + (i % width as usize) as f32 * size;
            let color = &self.genome[i];
            if quadrant.iter().any(|&x| std::ptr::eq(x, color)) {
                draw.rect()
                .x_y(x, y)
                .w_h(size, size)
                .color(Rgb::from_components(color.as_color()));
            }
        });
    }

    pub fn draw_simple_rect(&self, draw: &Draw, at: Vec2, size: f32) {
        let width = (self.genome.len() as f32).sqrt() as usize;
        let offset = (width as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);

        (0..width).for_each(|row| {
            let points = (0..width).map(|column| {
                let index = row * width + column;
                let x = bottom_left.0 + column as f32 * size;
                let y = bottom_left.1 + row as f32 * size;
                (
                    pt2(x, y),
                    Rgb::from_components(self.genome[index].as_color()),
                )
            });
            draw.polyline().weight(size).points_colored(points);
        })
    }

    pub fn draw_circle(&self, draw: &Draw) {
        let width = (self.genome.len() as f32).sqrt() as usize;
        let offset = (width as f32 / 2.) * self.nannou_size - self.nannou_size / 2.;
        let bottom_left = (
            self.nannou_position.x - offset,
            self.nannou_position.y - offset,
        );
        (0..self.genome.len()).for_each(|i| {
            let temp = i / width;
            let y = bottom_left.1 + (temp as f32) * self.nannou_size;
            let x = bottom_left.0 + (i % width as usize) as f32 * self.nannou_size;
            let dist = self.nannou_position.distance(Vec2::new(x, y));

            if dist <= offset {
                draw.rect()
                    .x_y(x, y)
                    .w_h(self.nannou_size, self.nannou_size)
                    .color(Rgb::from_components(self.genome[i].as_color()));
            }
        });
    }

    pub fn draw_simple_circle(&self, draw: &Draw) {
        let width = (self.genome.len() as f32).sqrt() as usize;
        let offset = (width as f32 / 2.) * self.nannou_size - self.nannou_size / 2.;
        let bottom_left = (
            self.nannou_position.x - offset,
            self.nannou_position.y - offset,
        );

        (0..width).for_each(|row| {
            let mut points = Vec::new();
            for column in 0..width {
                let index = row * width + column;
                let x = bottom_left.0 + column as f32 * self.nannou_size;
                let y = bottom_left.1 + row as f32 * self.nannou_size;
                let dist = self.nannou_position.distance(Vec2::new(x, y));

                if dist <= offset {
                    points.push((
                        pt2(x, y),
                        Rgb::from_components(self.genome[index].as_color()),
                    ))
                }
            }
            draw.polyline()
                .weight(self.nannou_size)
                .points_colored(points);
        })
    }
}
