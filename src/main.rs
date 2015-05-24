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

    let mut base_x = 32;
    let mut base_y = 61;

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
                Event::KeyDown { keycode: KeyCode::Up, .. }    => { base_y -= 16; },
                Event::KeyDown { keycode: KeyCode::Down, .. }  => { base_y += 16; },
                Event::KeyDown { keycode: KeyCode::Left, .. }  => { base_x -= 16; },
                Event::KeyDown { keycode: KeyCode::Right, .. } => { base_x += 16; },
                _ => {}
            }
        }
        sdl2::timer::delay(10);
        // The rest of the game loop goes here...

        drawer.set_draw_color(*back_color);
        drawer.clear();
        drawer.set_draw_color(*front_color);

        drawer.draw_1bpp(&nintendo, base_x, base_y, 96);
        drawer.present();
    }
}

trait Draw1BPP {
    fn draw_1bpp(&mut self, bytes: &[u8], origin_x: i32, origin_y: i32, width: i32);
}

impl<'a> Draw1BPP for sdl2::render::RenderDrawer<'a> {
    fn draw_1bpp(&mut self, bytes: &[u8], origin_x: i32, origin_y: i32, width: i32) {
        let mut i = 0;
        let mut x;
        let mut y;

        for byte in bytes.iter() {
            x = (i - (i % 8)) % width;
            y = (i / width) * 8 + i % 8;

            for b in 0..8 {
                if 1 == ((byte >> (7 - b)) & 1) {
                    self.draw_point(Point::new(origin_x + x + b, origin_y + y));
                }
            }

            i += 1;
        }
    }
}
