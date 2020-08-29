extern crate graphics;
use graphics::{Context, Graphics};
use std::collections::HashMap;
extern crate find_folder;
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

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{AdvancedWindow, Window, WindowSettings};
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
use std::*;
use touch_visualizer::TouchVisualizer;

type AxisValues = HashMap<(u32, u8), f64>;

pub fn initialize_window() {
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("go-rust", [400, 400])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let gl = &mut GlGraphics::new(opengl);
    let mut cursor = [0.0, 0.0];

    let mut _touch_visualizer = TouchVisualizer::new();
    let mut axis_values: AxisValues = HashMap::new();

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        _touch_visualizer.event(window.size(), &e);
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Turned capture cursor on");
                capture_cursor = !capture_cursor;
                window.set_capture_cursor(capture_cursor);
            }
            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(args) = e.button_args() {
            println!("Scancode {:?}", args.scancode);
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
                Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            }
        };
        if let Some(args) = e.controller_axis_args() {
            axis_values.insert((args.id, args.axis), args.position);
        }
        e.mouse_cursor(|pos| {
            cursor = pos;
            println!("Mouse moved '{} {}'", pos[0], pos[1]);
        });
        e.mouse_scroll(|d| println!("Scrolled mouse '{}, {}'", d[0], d[1]));
        e.mouse_relative(|d| println!("Relative mouse moved '{} {}'", d[0], d[1]));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|args| println!("Resized '{}, {}'", args.window_size[0], args.window_size[1]));
        if let Some(cursor) = e.cursor_args() {
            if cursor {
                println!("Mouse entered");
            } else {
                println!("Mouse left");
            }
        };
        if let Some(args) = e.render_args() {
            // println!("Render {}", args.ext_dt);
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([1.0; 4], g);
                draw_rectangles(&window, &c, g);
                draw_axis_values(&mut axis_values, &window, &c, g);
                _touch_visualizer.draw(&c, g);
                graphics::ellipse(
                    [0.0, 1.0, 0.0, 1.0],
                    graphics::ellipse::circle(cursor[0], cursor[1], 5.0),
                    c.transform,
                    g,
                );
            });
        }
        if let Some(_args) = e.idle_args() {
            // println!("Idle {}", _args.dt);
        }
        if let Some(_args) = e.update_args() {
            /*
            // Used to test CPU overload.
            println!("Update {}", _args.dt);
            let mut x: f64 = 0.0;
            for _ in 0..500_000 {
                x += (1.0 + x).sqrt();
            }
            println!("{}", x);
            */
        }
    }
}

pub fn draw_rectangles<G: Graphics>(window: &dyn Window, c: &Context, g: &mut G) {
    let size = window.size();
    let draw_size = window.draw_size();
    let zoom = 0.2;
    let offset = 30.0;

    let _rect_border = graphics::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);

    // Cursor.
    let _cursor_color = [0.0, 0.0, 0.0, 1.0];

    // Rectangle.
    let rect_border = graphics::Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 1.0);
    rect_border.draw(
        [
            offset + size.width as f64 * zoom,
            offset,
            draw_size.width as f64 * zoom,
            draw_size.height as f64 * zoom,
        ],
        &c.draw_state,
        c.transform,
        g,
    );
}

pub fn draw_axis_values<W: Window, G: Graphics>(
    axis_values: &mut AxisValues,
    window: &W,
    c: &Context,
    g: &mut G,
) {
    let window_height = window.size().height as f64;
    let max_axis_height = 200.0;
    let offset = 10.0;
    let top = window_height - (max_axis_height + offset);
    let color = [1.0, 0.0, 0.0, 1.0];
    let width = 10.0;
    let mut draw = |i, v: f64| {
        let i = i as f64;
        let height = (v + 1.0) / 2.0 * max_axis_height;
        let rect = [
            offset + i * (width + offset),
            top + max_axis_height - height,
            width,
            height,
        ];
        graphics::rectangle(color, rect, c.transform, g);
    };
    for (i, &v) in axis_values.values().enumerate() {
        draw(i, v);
    }
}
