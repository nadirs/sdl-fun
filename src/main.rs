extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::rect::*;

pub fn main() {
    let sdl_context = sdl2::init().video().unwrap();

    let window = sdl_context.window("Fun with Rust and SDL", 160*2, 140*2)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut renderer = window.renderer().build().unwrap();
    let mut drawer = renderer.drawer();

    let colors_vec = [Color::RGB(0, 0, 0), Color::RGB(0xff, 0xff, 0xff)];
    let mut colors = colors_vec.iter().cycle();

    let mut back_color = colors.next().unwrap();
    let mut front_color = colors.next().unwrap();

    drawer.set_scale(2f32, 2f32);

    let nintendo = [
        0xf0, 0xf0, 0xfc, 0xfc, 0xfc, 0xfc, 0xf3, 0xf3,
        0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c,
        0xf0, 0xf0, 0xf0, 0xf0, 0x00, 0x00, 0xf3, 0xf3,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0xcf,
        0x00, 0x00, 0x0f, 0x0f, 0x3f, 0x3f, 0x0f, 0x0f,
        0x00, 0x00, 0x00, 0x00, 0xc0, 0xc0, 0x0f, 0x0f,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xf0,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf3, 0xf3,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xc0,
        0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0xff, 0xff,
        0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc3, 0xc3,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfc,
        0xf3, 0xf3, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
        0x3c, 0x3c, 0xfc, 0xfc, 0xfc, 0xfc, 0x3c, 0x3c,
        0xf3, 0xf3, 0xf3, 0xf3, 0xf3, 0xf3, 0xf3, 0xf3,
        0xf3, 0xf3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3,
        0xcf, 0xcf, 0xcf, 0xcf, 0xcf, 0xcf, 0xcf, 0xcf,
        0x3c, 0x3c, 0x3f, 0x3f, 0x3c, 0x3c, 0x0f, 0x0f,
        0x3c, 0x3c, 0xfc, 0xfc, 0x00, 0x00, 0xfc, 0xfc,
        0xfc, 0xfc, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
        0xf3, 0xf3, 0xf3, 0xf3, 0xf3, 0xf3, 0xf0, 0xf0,
        0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xff, 0xff,
        0xcf, 0xcf, 0xcf, 0xcf, 0xcf, 0xcf, 0xc3, 0xc3,
        0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0xfc, 0xfc,
    ];

    drawer.set_draw_color(*front_color);

    let mut i = 0;
    let mut x;
    let mut y;

    let mut running = true;
    let mut event_pump = sdl_context.event_pump();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::*;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                Event::Window { win_event_id: event_id, .. } => {
                    match event_id {
                        WindowEventId::Enter | WindowEventId::Leave => {
                            back_color = front_color;
                            front_color = colors.next().unwrap();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        sdl2::timer::delay(10);
        // The rest of the game loop goes here...

        drawer.set_draw_color(*back_color);
        drawer.clear();
        drawer.set_draw_color(*front_color);
        for byte in nintendo.iter() {
            x = (i - (i % 8)) % 96;
            y = (i / 96) * 8 + i % 8;

            for b in 0..8 {
                if 1 == ((byte >> (7 - b)) & 1) {
                    drawer.draw_point(Point::new(32 + x + b, 61 + y));
                }
            }

            i += 1;
        }
        i = 0;
        drawer.present();
    }
}