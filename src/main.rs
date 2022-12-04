use itertools::Itertools;
use nannou::prelude::*;
use rand::random;

const SCALE: f32 = 16.0;
const GRID_SIZE_X: f32 = 60.0;
const GRID_SIZE_Y: f32 = 40.0;
const UPDATE_TIME: f32 = 0.1;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Copy, Clone)]
struct Canvas {
    elements: [[u8; GRID_SIZE_Y as usize]; GRID_SIZE_X as usize],
}

impl Canvas {
    fn new() -> Self {
        let mut temp: Canvas = Canvas {
            elements: [[0; GRID_SIZE_Y as usize]; GRID_SIZE_X as usize],
        };

        // for ((x,y),element) in &mut temp.iter_elements() {
        //     *element = random::<u8>() % 2;
        // }

        for x in 0..temp.elements.len() as usize {
            for y in 0..temp.elements[0].len() as usize {
                temp.elements[x][y] = random::<u8>() % 2;
            }
        }
        temp
    }

    pub fn iter_elements(&self) -> impl Iterator<Item = ((usize, usize), &u8)> {
        self.elements.iter().enumerate().flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(move |(y, column)| ((x, y), column))
        })
    }

    fn get_value(&self, mut x: isize, mut y: isize) -> u8 {
        x = (x + self.elements.len() as isize) % self.elements.len() as isize;
        y = (y + self.elements[0].len() as isize) % self.elements[0].len() as isize;
        self.elements[x as usize][y as usize]
    }

    fn get_neighbors(&self, x: usize, y: usize) -> usize {
        let mut number: u8 = 0;

        for (dx, dy) in [-1, 0, 1].iter().cartesian_product([-1, 0, 1].iter()) {
            if *dx != 0 || *dy != 0 {
                number += self.get_value(x as isize + dx, y as isize + dy);
            }
        }
        number as usize
    }
}

struct Model {
    screen: Canvas,
    t: f32,
}

impl Model {
    fn new() -> Self {
        Self {
            screen: Canvas::new(),
            t: 0.0,
        }
    }

    fn draw(&self, draw: &Draw) {
        let color = [GRAY, YELLOW];
        for ((x, y), element) in self.screen.iter_elements() {
            draw.rect()
                .x_y(x as f32, y as f32)
                .w_h(0.9, 0.9)
                .color(color[*element as usize]);
        }
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // Handle time
    if app.time - m.t > UPDATE_TIME {
        m.t = app.time;
        // Get current stage
        let leap = m.screen;

        for ((x, y), element) in leap.iter_elements() {
            m.screen.elements[x][y] = match leap.get_neighbors(x, y) {
                0 | 1 | 4 | 5 | 6 | 7 | 8 => 0, // Death
                n @ 2..=3 => {
                    if *element == 0 && n == 2 {
                        0 // stay dead
                    } else {
                        1 // live
                    }
                }
                _ => 0,
            }
        }
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
