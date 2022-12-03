use itertools::Itertools;
use nannou::prelude::*;

const SCALE: f32 = 20.0;
const GRID_SIZE_X: f32 = 40.0;
const GRID_SIZE_Y: f32 = 40.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Canvas {
    canvas: [[u8; GRID_SIZE_X as usize]; GRID_SIZE_Y as usize],
}

impl Canvas {
    fn new() -> Self {
        Self {
            canvas: [[0; GRID_SIZE_X as usize]; GRID_SIZE_Y as usize],
        }
    }

    fn get_value(&self, x: isize, y: isize) -> u8 {
        x = (x + GRID_SIZE_X as isize) % GRID_SIZE_X as isize;
        y = (y + GRID_SIZE_Y as isize) % GRID_SIZE_Y as isize;
        self.canvas[x as usize][y as usize]
    }

    fn getNeighbors(&self, x: usize, y: usize) -> usize {
        let mut number: u8 = 0;

        for (dx, dy) in [-1, 0, 1].iter().cartesian_product([-1, 0, 1].iter()) {
            number += self.get_value(x as isize + dx, y as isize + dy);
        }
        number as usize
    }
}

struct Model {
    screen: Canvas,
}

impl Model {
    fn new() -> Self {
        Self {
            screen: Canvas::new(),
        }
    }

    fn draw(&self, draw: &Draw) {
        for y in 0..GRID_SIZE_Y as usize {
            for x in 0..GRID_SIZE_X as usize {
                draw.rect()
                    .x_y(x as f32, y as f32)
                    .w_h(0.9, 0.9)
                    .color(self.screen.canvas[x][y]);
            }
        }
    }
}

fn update(_app: &App, _m: &mut Model, _update: Update) {}

fn event(_app: &App, _m: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(_button) => {}
        _other => (),
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(
            GRID_SIZE_X as u32 * SCALE as u32,
            GRID_SIZE_Y as u32 * SCALE as u32,
        )
        .view(view)
        .event(event)
        .build()
        .unwrap();
    Model::new()
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app
        .draw()
        .scale(SCALE)
        .x_y(-((GRID_SIZE_X / 2.0) - 0.5), -GRID_SIZE_Y / 2.0 + 0.5);
    draw.background().color(LIGHTGRAY);
    // Draw field
    m.draw(&draw);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
