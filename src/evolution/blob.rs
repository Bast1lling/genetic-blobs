use nannou::color::Rgb;
use nannou::Draw;
use nannou::geom::{Point2, pt2, Vec2};
use crate::{Model, Nannou};
/// external crate
use rand::{Rng, thread_rng};

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

#[derive(Debug, Clone)]
pub struct Blob {
    data: Vec<RGB>,
    size: usize,
    gui_size: f32,
    gui_position: Point2,
}

/// A Genome is a set of single data blocks
pub trait Gene {

    /// determines the fitness of the full genome
    /// the larger the result, the better is the genome
    /// currently no normalization is required
    fn rate_fitness(&self) -> f32;

    /// determines the fitness of a single genome building block
    fn rate_fitness_at(&self, at: usize) -> f32;

    /// replaces a single genome building block by a random block 
    fn mutate(&mut self, at: usize);

    /// returns the amount of single building blocks
    fn length(&self) -> usize;

    /// combines parts of multiple genes into a new one:
    /// mother: Genome from which also all non-genetic information will be copied
    /// fathers: {f | f is Genome} 
    /// indices: i: Blocks in Genome -> parent in {mother, fathers}
    fn combine(mother: &Self, fathers: &Vec<Self>, indices: &Vec<u8>) -> Self
    where Self: Sized;
}

impl Gene for Blob {


    fn rate_fitness(&self) -> f32 {
        // let weight = 1.0/(3.0 * SIZE as f32 * SIZE as f32);
        let treshold: u8 = 15;
        let mut fitness = 0.0;
        for color in &self.data {
            if color.r < treshold && color.g < treshold && color.b < treshold {
                fitness += 1.0;
            }
        }
        fitness
    }
    
    fn rate_fitness_at(&self, at: usize) -> f32 {
        // let weight = 1.0/(3.0 * SIZE as f32 * SIZE as f32);
        let color: &RGB = &self.data[at];
        let treshold: u8 = 15;
        if color.r < treshold && color.g < treshold && color.b < treshold {
            1.0
        }
        else {
            0.0
        }
    }
    
    fn length(&self) -> usize {
        self.size
    }
    
    fn mutate(&mut self, at: usize) {
        let f = Blob::color_generator();
        self.data[at] = f();
    }
    
    fn combine(mother: &Self, fathers: &Vec<Self>, indices: &Vec<u8>) -> Self
    where Self: Sized {
        assert!(mother.data.len() == indices.len());

        let mut child = mother.clone();

        for (at, from) in indices.iter().enumerate() {
            let from = *from as usize;
            if from >= fathers.len() {
                continue;
            }
            child.data[at] = fathers[from].data[at];
        }
        child
    }
}

impl Nannou for Blob {
    fn draw(&self, draw: &Draw, model: &Model) {
        let position = model.transform(self.gui_position);
        let size = self.gui_size * model.zoom;
        let mode = 0;

        match mode {
            0 => self.draw_rect(draw, position, size),
            1 => self.draw_simple_rect(draw, position, size),
            2 => self.draw_circle(draw),
            3 => self.draw_simple_circle(draw),
            _ => panic!("This drawing mode does not exist!")
        }
    }

    fn update(&mut self) {

    }
}

impl Blob {
    pub fn color_generator() -> impl Fn() -> RGB {
        || {
            let mut rng = thread_rng();
            RGB { r: rng.gen(), g: rng.gen(), b: rng.gen() }
        }
    }

    pub fn new<F>(size: usize, gui_size: f32, gui_position: Point2, color_generator: F) -> Self
    where F: Fn() -> RGB{
        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(color_generator());
        }

        Self { data, size, gui_size, gui_position }
    }

    pub fn draw_rect(&self, draw: &Draw, at: Vec2, size: f32) {
        let offset = (SIZE as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);
        (0..(SIZE * SIZE)).for_each(|i| {
            let temp = i / SIZE;
            let y = bottom_left.1 + temp as f32 * size;
            let x = bottom_left.0 + (i % SIZE) as f32 * size;
            draw.rect().
                x_y(x,y).
                w_h(size,size).
                color(Rgb::from_components(self.data[i].as_color()));
        });
    }

    pub fn draw_simple_rect(&self, draw: &Draw, at: Vec2, size: f32) {
        let offset = (SIZE as f32 / 2.) * size - size / 2.;
        let bottom_left = (at.x - offset, at.y - offset);

        (0..SIZE).for_each(|row| {
            let points = (0..SIZE).map(|column|{
                let index = row * SIZE + column;
                let x = bottom_left.0 + column as f32 * size;
                let y = bottom_left.1 + row as f32 * size;
                (pt2(x, y), Rgb::from_components(self.data[index].as_color()))
            });
            draw.polyline().weight(size).points_colored(points);
        })
    }

    pub fn draw_circle(&self, draw: &Draw) {
        let offset = (SIZE as f32 / 2.) * self.gui_size - self.gui_size / 2.;
        let bottom_left = (self.gui_position.x - offset, self.gui_position.y - offset);
        (0..(SIZE * SIZE)).for_each(|i| {
            let temp = i / SIZE;
            let y = bottom_left.1 + temp as f32 * self.gui_size;
            let x = bottom_left.0 + (i % SIZE) as f32 * self.gui_size;
            let dist = self.gui_position.distance(Vec2::new(x,y));

            if dist <= offset {
                draw.rect().
                    x_y(x,y).
                    w_h(self.gui_size,self.gui_size).
                    color(Rgb::from_components(self.data[i].as_color()));
            }
        });
    }

    pub fn draw_simple_circle(&self, draw: &Draw) {
        let offset = (SIZE as f32 / 2.) * self.gui_size - self.gui_size / 2.;
        let bottom_left = (self.gui_position.x - offset, self.gui_position.y - offset);

        (0..SIZE).for_each(|row| {
            let mut points = Vec::new();
            for column in 0..SIZE {
                let index = row * SIZE + column;
                let x = bottom_left.0 + column as f32 * self.gui_size;
                let y = bottom_left.1 + row as f32 * self.gui_size;
                let dist = self.gui_position.distance(Vec2::new(x,y));

                if dist <= offset {
                    points.push((pt2(x,y), Rgb::from_components(self.data[index].as_color())))
                }
            }
            draw.polyline().weight(self.gui_size).points_colored(points);
        })
    }
}