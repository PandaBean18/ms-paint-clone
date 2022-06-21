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
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;
use crate::piston::Window as OtherWindow;
use piston::input::*;
use piston::Size;

pub struct App {
    gl: GlGraphics, 
}

// struct for all the brush....strokes?
pub struct AppSquare {
    x: f64, 
    y: f64, 
    side: f64,
    color: [f32; 4],
}

// Color palette at the bottom
pub struct ColorSelector {
    x: f64, 
    y: f64, 
    color: [f32; 4]
}

impl App {
    fn render(&mut self, args: &RenderArgs, squares: &Vec<AppSquare>, cur: &mut AppSquare, palette: &Vec<ColorSelector>) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; 

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl); 
            // drawing the canvas border
            let canvas_border = Rectangle::new_border(color::BLACK, 1.0);
            canvas_border.draw([0.0, 0.0, 600.0, 400.0], 
                &draw_state::DrawState::default(), 
                c.transform.trans((args.window_size[0] - 600.0) / 2.0, 50.0), // (args.window_size[0] - 600.0) / 2.0 is done to 
                // ensure it is in center
                gl
            );

            // drawing all the colors in the color palette along with their borders 
            for col in palette {
                let color_picker_tray = Rectangle::new_border(color::BLACK, 1.0); 
                let transform = c.transform.trans(col.x , col.y); // transform is actually what defines the coords of where rect is drawn
                color_picker_tray.draw([0.0, 0.0, 30.0, 30.0], 
                    &draw_state::DrawState::default(), // default drawstate
                    transform, 
                    gl
                );

                let square = rectangle::square(0.0, 0.0, 29.0);
                rectangle(col.color, square, transform, gl);
            }

            let x: f64; 
            let y: f64; 

            // all the if statements here are to ensure that you cant draw outside the canvas
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
            
            // Basically all the positions in which the mouse's left button is pressed, are stored in a vector, 
            // these are then drawn on the screen one after another
            // This is HIGHLY ineffcient but it was the first thing that came to my mind and works for a project of this scale
            // Piston examples on github has a guide on how to create paint properly.
            for sq in squares {
                let x = sq.x; 
                let y = sq.y; 

                let transform = c.transform.trans(x, y-(sq.side / 2.0));
                let square = rectangle::square(0.0, 0.0, sq.side);
                rectangle(sq.color, square, transform, gl);
            }
            
        });
        

    }

    // function to check if cursor is inside canvas or not
    fn cursor_inside_canvas(&self, cur: &AppSquare, window_size: &Size) -> bool {
        if !(cur.x >= (window_size.width - 600.0) / 2.0 && cur.x + (cur.side) <= ((window_size.width - 600.0) / 2.0) + 600.0) {
            return false;
        }

        if !(cur.y - (cur.side / 2.0) >= 50.0 && cur.y + (cur.side / 2.0) <= 450.0) {
            return false;
        }

        return true;
    }

    // function to check if cursor is inside color palette or not
    fn cursor_on_palette(&self, cur: &AppSquare, window_size: &Size) -> bool {
        if !(cur.x >= (window_size.width - 150.0) / 2.0 && cur.x + cur.side <= ((window_size.width - 150.0) / 2.0) + 150.0) {
            return false;
        } 
        if !(cur.y - (cur.side / 2.0) >= window_size.height - 50.0 && cur.y + (cur.side / 2.0) <= window_size.height - 20.0) {
            return false;
        }

        return true;
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("test", [800, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl), 
    };

    // cursor
    let mut cur = AppSquare {
        x: 0.0, 
        y: 0.0, 
        color: [1.0, 0.0, 0.0, 1.0],
        side: 5.0,
    };

    let mut events = Events::new(EventSettings::new());
    let mut touch_visualizer = TouchVisualizer::new();
    let mut cursor: [f64; 2] = [0.0, 0.0]; // is actually cursor position
    let mut button_state: ButtonState = ButtonState::Release; // this variable will basically store button state, this is done because
    // while going through piston examples i could not find a way to work with button long press event (ifykwim) so i created this variable
    // it basically gets set to ButtonState::Press when button is pressed, and to ButtonState::Release when it is released. Working this
    // way lets the program know when to draw and when not to

    let mut prev_mouse_button: Option<MouseButton> = None; // stores last pressed button, dont wanna draw when right button was pressed
    let mut squares: Vec<AppSquare> = vec![];

    while let Some(e) = events.next(&mut window) {
        // The color palette vector is inside the while loop because i want the color palette to be centered at all times
        // and that requires computing values of x and y on each iteration.
        let color_palette = vec![
            ColorSelector {
                x: (window.size().width - 150.0) / 2.0, 
                y: window.size().height - 50.0,
                color: graphics::color::RED, 
            }, 
            ColorSelector {
                x: ((window.size().width - 150.0) / 2.0) + 30.0, 
                y: window.size().height - 50.0,
                color: graphics::color::GREEN, 
            }, 
            ColorSelector {
                x: ((window.size().width - 150.0) / 2.0) + 60.0, 
                y: window.size().height - 50.0,
                color: graphics::color::BLUE, 
            },
            ColorSelector {
                x: ((window.size().width - 150.0) / 2.0) + 90.0, 
                y: window.size().height - 50.0,
                color: graphics::color::YELLOW, 
            },
            ColorSelector {
                x: ((window.size().width - 150.0) / 2.0) + 120.0,  
                y: window.size().height - 50.0,
                color: graphics::color::WHITE, 
            }
        ];

        touch_visualizer.event(window.size(), &e);

        // updating cursor pos on each iteration
        e.mouse_cursor(|pos| {
            cursor = pos;
            cur.x = pos[0] - (cur.side / 2.0); 
            cur.y = pos[1];
        });

        // cursor resizing logic lul
        e.mouse_scroll(|d| {
            if d[1] < 0.0 && cur.side > 5.0 {
                cur.side += 5.0 * d[1];
            } else if d[1] > 0.0 && cur.side < 50.0 {
                cur.side += 5.0 * d[1];
            }
        });

        // updating button state
        e.button(|args| {
            button_state = args.state;
        });  

        // matching button state to do stuff when its pressed
        match button_state {
            ButtonState::Press => {
                match prev_mouse_button {
                    None => {}
                    Some(MouseButton::Left) => {
                        if app.cursor_inside_canvas(&cur, &window.size()) {
                            let mut sq = AppSquare {
                                x: cursor[0]-(cur.side / 2.0), 
                                y: cursor[1], 
                                color: cur.color, 
                                side: cur.side, 
                            };

                            squares.push(sq);
                        }

                        if app.cursor_on_palette(&cur, &window.size()) {
                            cur.color = color_palette[((cursor[0] - (window.size().width - 150.0) / 2.0) / 30.0) as usize].color;
                        }
                    }
                    _ => {}
                }
            }

            ButtonState::Release => {}
        }

        // updating prev_mouse_button
        if let Some(Button::Mouse(button)) = e.press_args() {
            prev_mouse_button = Some(button);
        }

        if let Some(_button) = e.release_args() {
            prev_mouse_button = None; 
        }

        // rendering stuff 
        if let Some(args) = e.render_args() {
            app.render(&args, &squares, &mut cur, &color_palette);
        }

    }
}
