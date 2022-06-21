extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate touch_visualizer;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;

use touch_visualizer::TouchVisualizer;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::piston::Window as OtherWindow;
use piston::input::*;

pub struct App {
    gl: GlGraphics, 
}

pub struct AppSquare {
    x: f64, 
    y: f64, 
    side: f64,
    color: [f32; 4],
    dir: Direction
}

 #[derive(PartialEq)]
enum Direction {
    Up, 
    Down, 
    Left, 
    Right, 
    None
}
impl App {
    fn render(&mut self, args: &RenderArgs, squares: &Vec<AppSquare>, cur: &mut AppSquare) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; 

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl); 

            let x: f64; 
            let y: f64; 

            if cur.x < 0.0 {
                x = args.window_size[0] - (cur.x % args.window_size[0]).abs(); 
            } else {
                x = cur.x % args.window_size[0];
            }

            if cur.y < 0.0 {
                y = args.window_size[1] - (cur.y % args.window_size[1]).abs();
            } else { 
                y = cur.y % args.window_size[1];
            }

            let transform = c.transform.trans(x, y-(cur.side / 2.0));
            let square = rectangle::square(0.0, 0.0, cur.side);
            rectangle(cur.color, square, transform, gl);
            
            for sq in squares {
                let x = sq.x; 
                let y = sq.y; 

                let transform = c.transform.trans(x, y-(sq.side / 2.0));
                let square = rectangle::square(0.0, 0.0, sq.side);
                rectangle(sq.color, square, transform, gl);
            }
            
        });
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("test", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl), 
    };

    let mut cur = AppSquare {
        x: 0.0, 
        y: 0.0, 
        color: [1.0, 0.0, 0.0, 1.0],
        side: 5.0,
        dir: Direction::None
    };

    let mut events = Events::new(EventSettings::new());
    let mut touch_visualizer = TouchVisualizer::new();
    let mut cursor: [f64; 2] = [0.0, 0.0];
    let mut button_state: ButtonState = ButtonState::Release;
    let mut prev_mouse_button: Option<MouseButton> = None; 
    let mut prev_key: Option<Key> = None;
    let mut squares: Vec<AppSquare> = vec![];

    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);
        
        e.mouse_cursor(|pos| {
            cursor = pos;
            cur.x = pos[0] - (cur.side / 2.0); 
            cur.y = pos[1];
        });

        e.mouse_scroll(|d| {
            if d[1] < 0.0 && cur.side > 5.0 {
                cur.side += 5.0 * d[1];
            } else if d[1] > 0.0 && cur.side < 50.0 {
                cur.side += 5.0 * d[1];
            }
        });

        e.button(|args| {
            button_state = args.state;
        });  

        match button_state {
            ButtonState::Press => {
                match prev_mouse_button {
                    None => {}
                    Some(MouseButton::Left) => {
                        let mut sq = AppSquare {
                            x: cursor[0]-(cur.side / 2.0), 
                            y: cursor[1], 
                            color: cur.color, 
                            side: cur.side, 
                            dir: Direction::None
                        };

                        squares.push(sq);
                    }
                    _ => {}
                }
            }

            ButtonState::Release => {}
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            prev_mouse_button = Some(button);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            prev_key = Some(key);
        }

        if let Some(_button) = e.release_args() {
            prev_mouse_button = None; 
            prev_key = None;
        }

        if let Some(args) = e.render_args() {
            app.render(&args, &squares, &mut cur);
        }

    }
}
