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
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

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
        const DIAMOND: [types::Vec2d; 6] = [
          [-2.0,  0.0],
          [-1.0,  1.0],
          [ 1.0,  1.0],
          [ 2.0,  0.0],
          [ 1.0, -1.0],
          [-1.0, -1.0],
        ];


        let hexagon: [types::Vec2d; 6] = generate_hexagon();

        let somepoly: graphics::types::Polygon = &hexagon;

        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
        let current_scale: types::Vec2d = [10.0, 10.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);
            let mut foreground_index = 0;

            let mut cursor: types::Vec2d = [0.0, 0.0];
            let cell_size: f64 = 30.0;
            for row in (0..30) {
              cursor[0] = if row % 2 == 1 { 0.0 } else { cell_size / 2.0 };
              cursor[1] += 9.0;
      

              for col in (0..30) {
                cursor[0] += cell_size;

                let transform = c.transform
                                 .trans(cursor[0], cursor[1])
                                 .scale(10.0, 10.0)
                ;
                polygon(FOREGROUNDS[foreground_index], somepoly, transform, gl);
                foreground_index += 1;
                if foreground_index == 7 { foreground_index = 0 };
              }
            }
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

}

fn generate_hexagon() -> [types::Vec2d; 6] {
  use std::f64::*;
  let mut hex: [types::Vec2d;6] = [[0.0, 0.0]; 6];
  let scale:f64 = 1.0;

  for (i, coord) in hex.iter_mut().enumerate() {
    let n = (i + 1) as f64;
    let point: f64 = 2.0 * consts::PI * n/6.0;
    coord[0] = scale * point.cos();
    coord[1] = scale * point.sin();
  }
  hex
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Hex Party",
            [1200, 1200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
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
