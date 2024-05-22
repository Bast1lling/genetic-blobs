mod evolution;
mod statistics;
mod simulation;
mod util;

/// nannou
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

/// internal modules
use evolution::{
    implementations::simple::SimpleBlobPopulation,
    blob::{Blob, SIZE},
};
use util::distribute_uniformly;

use crate::evolution::{blob::RGB, gene::Evolve};

/// STARTING WINDOW SIZE
const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

trait Nannou {
    fn draw(&self, draw: &Draw, model: &Model);
    fn update(&mut self);
}
struct Model {
    center: Point2,
    zoom: f32,
    population: SimpleBlobPopulation,
    egui: Egui,
    count: u32,
    window_id: WindowId,
}

impl Model {
    fn transform(&self, v: Vec2) -> Vec2 {
        (v - self.center) * self.zoom
    }
}

fn main() {
    nannou::app(model).update(update).run();
    //avg(&get_grades());
}

fn model(app: &App) -> Model {

    let genome_length = SIZE * SIZE;
    let blob_size = 3.0;
    let blob_amount: u16 = 64;
    let points = distribute_uniformly(blob_amount, (genome_length as f32).sqrt() * blob_size);

    let population = SimpleBlobPopulation::new(points, blob_size, blob_amount, genome_length);

    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .mouse_wheel(mouse_wheel)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let count = 128;
    let position = Point2::new(0., 0.);

    Model {
        center: position,
        zoom: 1.0,
        population,
        egui,
        count,
        window_id,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model { 
        ref mut egui,
        ref mut population,
        .. 
    } = *model;
    
    //SimpleBlobPopulation::evolve(blobs.population.iter_mut().map(|b| b.genome).collect());
    population.update();
    println!("FPS: {}", app.fps());
    //if app.time.round() as i32 % 5 == 0 {
    //}

    let pos_shift = scroll(app, model.window_id, app.mouse.position());
    model.center += pos_shift;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Workshop window").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut model.count, 0..=2000).text("circle count"))
            .changed();
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let _window = app.window(model.window_id).unwrap();
    draw.background().rgb(0.11, 0.12, 0.13);

    //draw_coordinate_system(&app, &model, &draw, &window);
    //model.blob.draw_as_polyline(&draw, (0., 0.), 30.);
    //draw_function(&draw, &win, |x| (1./win.h()) * x * x, 1.);

    model.population.draw(&draw, model);

    draw.to_frame(app, &frame).unwrap();

    // let fr = app.fps();
    //println!("framerate: {fr}");

    let _ = model.egui.draw_to_frame(&frame);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn mouse_wheel(_app: &App, model: &mut Model, dt: MouseScrollDelta, _phase: TouchPhase) {
    let zoom_change = zoom(dt, model.zoom);
    model.zoom = (model.zoom + zoom_change).clamp(0.25, 4.0);
}

fn scroll(app: &App, window_id: WindowId, pos: Point2) -> Vec2 {
    let window = app.window(window_id).unwrap();
    let pad = 50.0;
    let step_size = 10.0;
    let rect = window.rect().pad(pad);
    let mut vec = Vec2::new(0.0, 0.0);
    if pos.x > rect.right() || pos.x < rect.left() || pos.y > rect.top() || pos.y < rect.bottom() {
        vec = Vec2::new(pos.x, pos.y);
        vec = vec.normalize();
        vec *= step_size;
    }
    vec
}

fn zoom(dt: MouseScrollDelta, current: f32) -> f32 {
    match dt {
        MouseScrollDelta::LineDelta(_, y) => 0.03 * current * y,
        MouseScrollDelta::PixelDelta(_) => 0.0,
    }
}
