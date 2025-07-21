mod conway;

use conway::FrameBuffer;
use raylib::prelude::*;

fn main() {
    let (width, height) = (800, 600);
    let scale = 8;
    let (cols, rows) = (width / scale, height / scale);

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Conway's Game of Life - Rust + Raylib")
        .build();

    rl.set_target_fps(10);

    let mut fb = FrameBuffer::new(cols, rows);

    while !rl.window_should_close() {
        conway::initialize_pattern(&mut fb);
        fb.render();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..fb.height {
            for x in 0..fb.width {
                let color = fb.get_color(x, y);
                if color != Color::BLACK {
                    d.draw_rectangle(x * scale, y * scale, scale, scale, Color::PURPLE);
                }
            }
        }
    }
}
