extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate touch_visualizer;

#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;
#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;

#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;

use graphics::line::*;
use graphics::{Context, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{AdvancedWindow, Window, WindowSettings};
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
use std::*;
use touch_visualizer::TouchVisualizer;

use crate::data::Board;
use crate::data::Value;

pub fn initialize_window(board: &Board) {
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("go-rust", [650, 650])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .resizable(false)
        .build()
        .unwrap();

    let mut capture_cursor = false;
    let gl = &mut GlGraphics::new(opengl);
    let mut cursor = [0.0, 0.0];

    let mut _touch_visualizer = TouchVisualizer::new();

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        _touch_visualizer.event(window.size(), &e);
        if let Some(Button::Mouse(button)) = e.press_args() {
            // println!("Pressed mouse button '{:?}'", button);
        }

        e.mouse_cursor(|pos| {
            cursor = pos;
            // println!("Mouse moved '{} {}'", pos[0], pos[1]);
        });
        // e.mouse_scroll(|d| println!("Scrolled mouse '{}, {}'", d[0], d[1]));
        // e.mouse_relative(|d| println!("Relative mouse moved '{} {}'", d[0], d[1]));
        // e.text(|text| println!("Typed '{}'", text));
        e.resize(|args| println!("Resized '{}, {}'", args.window_size[0], args.window_size[1]));
        if let Some(cursor) = e.cursor_args() {
            if cursor {
                println!("Mouse entered");
            } else {
                println!("Mouse left");
            }
        };

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                draw_grid(&c, g, &board);
                _touch_visualizer.draw(&c, g);
                graphics::ellipse(
                    [0.0, 1.0, 0.0, 1.0],
                    graphics::ellipse::circle(cursor[0], cursor[1], 5.0),
                    c.transform,
                    g,
                );
            });
        }
    }
}

pub fn draw_grid<G: Graphics>(c: &Context, g: &mut G, board: &Board) {
    let size = board.0.len() * 20;

    let field_rect = vec![25, 25, size, size];

    let cell_w = size / board.0.len();
    let cell_h = size / board.0.len();

    // Line
    for i in 0..board.0.len() {
        graphics::line(
            [0.5, 0.5, 0.5, 1.0],
            2.0,
            [
                (field_rect[0] + i * cell_w) as f64,
                field_rect[1] as f64,
                (field_rect[0] + i * cell_w) as f64,
                (field_rect[1] + field_rect[3]) as f64,
            ],
            c.transform,
            g,
        );
    }

    //Line
    for i in 0..board.0.len() {
        graphics::line(
            [0.5, 0.5, 0.5, 1.0],
            2.0,
            [
                field_rect[0] as f64,
                (field_rect[1] + i * cell_h) as f64,
                (field_rect[0] + field_rect[2]) as f64,
                (field_rect[1] + i * cell_h) as f64,
            ],
            c.transform,
            g,
        );
    }
}
