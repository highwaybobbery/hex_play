extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

pub struct Dimension {
  width: f64,
  height: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // args has .width and .height of current app screen size fyi

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const FOREGROUNDS: [[f32; 4];7] = [
          [1.0, 1.0, 1.0, 0.1],
          [1.0, 1.0, 1.0, 0.2],
          [1.0, 1.0, 1.0, 0.3],
          [1.0, 1.0, 1.0, 0.4],
          [1.0, 1.0, 1.0, 0.5],
          [1.0, 1.0, 1.0, 0.6],
          [1.0, 1.0, 1.0, 0.7],
        ];
        const GRID_COLS: i32 = 6;
        // 34 @ 10 cols;, 104 @ 30 cols
        const GRID_ROWS: i32 = (GRID_COLS as f64 * 3.5 - 0.75) as i32;

        let smallest_dimension = std::cmp::min(args.width, args.height) as f64;
        let grid_size = smallest_dimension / (GRID_COLS as f64 * 1.5 + 0.25);

        let mut hexagon_matrix: [types::Vec2d; 6] = [[0.0, 0.0]; 6];
        regular_polygon(&mut hexagon_matrix, grid_size / 2.0);
        let hexagon: graphics::types::Polygon = &hexagon_matrix;
        let tile_dimensions: Dimension = get_dimensions(hexagon);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            let mut tile_index = 0;

            let mut cursor: types::Vec2d = [0.0, 0.0];
            for row in 0..GRID_ROWS {

              let col_indent = if row % 2 == 0 {
                tile_dimensions.width * 0.5
              } else {
                tile_dimensions.width * 1.25
              };

              cursor[1] += tile_dimensions.height / 2.0;

              for col in 0..GRID_COLS {
                cursor[0] = col as f64 * (tile_dimensions.width * 1.5) + col_indent;

                let transform = c.transform.trans(cursor[0], cursor[1]);

                polygon(FOREGROUNDS[tile_index % FOREGROUNDS.len()], hexagon, transform, gl);
                tile_index += 1;
              }
            }
        });

    }
    fn update(&mut self, _: &UpdateArgs) {

    }

}

fn regular_polygon (points: &mut[[f64;2]], radius: f64 ) {
  use std::f64::*;
  let sides = points.len() as f64;
  for (i, coord) in points.iter_mut().enumerate() {
    let n = (i + 1) as f64;
    let point: f64 = 2.0 * consts::PI * n/sides;
    coord[0] = radius * point.cos();
    coord[1] = radius * point.sin();
  }
}

fn get_dimensions(polygon: graphics::types::Polygon) -> Dimension {
  let mut width_min = std::f64::MAX;
  let mut width_max = std::f64::MIN;
  let mut height_min = std::f64::MAX;
  let mut height_max = std::f64::MIN;

  for point in polygon.iter() {
    if point[0] > width_max { width_max = point[0] }
    if point[0] < width_min { width_min = point[0] }
    if point[1] > height_max { height_max = point[1] }
    if point[1] < height_min { height_min = point[1] }
  }

  Dimension { width: width_max - width_min, height: height_max - height_min }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Hex Party",
            [500, 500]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
