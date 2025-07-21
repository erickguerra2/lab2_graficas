use raylib::prelude::*;

pub struct FrameBuffer {
    pub buffer: Vec<Color>,
    pub width: i32,
    pub height: i32,
    pub frame_count: u32,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            buffer: vec![Color::BLACK; (width * height) as usize],
            width,
            height,
            frame_count: 0,
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.buffer[(y * self.width + x) as usize] = color;
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.buffer[(y * self.width + x) as usize]
        } else {
            Color::BLACK
        }
    }

    pub fn render(&mut self) {
        let mut next = self.buffer.clone();

        let neighbors = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        for y in 0..self.height {
            for x in 0..self.width {
                let mut alive_neighbors = 0;

                for (dx, dy) in &neighbors {
                    let nx = x + dx;
                    let ny = y + dy;
                    if self.get_color(nx, ny) == Color::WHITE {
                        alive_neighbors += 1;
                    }
                }

                let idx = (y * self.width + x) as usize;
                let cell = self.get_color(x, y) == Color::WHITE;

                next[idx] = match (cell, alive_neighbors) {
                    (true, 2) | (true, 3) => Color::WHITE,
                    (false, 3) => Color::WHITE,
                    _ => Color::BLACK,
                };
            }
        }

        self.buffer = next;
        self.frame_count += 1;
    }
}

fn spawn<F: FnMut(i32, i32)>(coords: &[(i32, i32)], ox: i32, oy: i32, mut set: F) {
    for (dx, dy) in coords {
        set(ox + dx, oy + dy);
    }
}

pub fn glider(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(1,0),(2,1),(0,2),(1,2),(2,2)], x, y, set);
}

pub fn lwss(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(1,0),(2,0),(3,0),(4,0),(0,1),(4,1),(4,2),(0,3),(3,3)], x, y, set);
}

pub fn mwss(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(1,0),(2,0),(3,0),(4,0),(5,0),(0,1),(5,1),(5,2),(0,3),(4,3)], x, y, set);
}

pub fn pulsar(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    let offsets = [
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12)
    ];
    spawn(&offsets, x, y, set);
}

pub fn beacon(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(0,0),(1,0),(0,1),(1,1),(2,2),(3,2),(2,3),(3,3)], x, y, set);
}

pub fn pentadecathlon(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(2,0),(2,1),(1,2),(2,2),(3,2),(2,3),(2,4),(2,5),(1,6),(2,6),(3,6),(2,7),(2,8)], x, y, set);
}

pub fn block(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(0,0),(1,0),(0,1),(1,1)], x, y, set);
}

pub fn loaf(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(1,0),(2,0),(0,1),(3,1),(1,2),(3,2),(2,3)], x, y, set);
}

pub fn boat(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(0,0),(1,0),(0,1),(2,1),(1,2)], x, y, set);
}

pub fn tub(x: i32, y: i32, set: &mut dyn FnMut(i32, i32)) {
    spawn(&[(1,0),(0,1),(2,1),(1,2)], x, y, set);
}

pub fn initialize_pattern(fb: &mut FrameBuffer) {
    let width = fb.width;
    let height = fb.height;

    if fb.frame_count == 0 {
        let mut set = |x: i32, y: i32| {
            if x >= 0 && y >= 0 && x < width && y < height {
                fb.point(x, y, Color::WHITE);
            }
        };

        let spacing_x = width / 4;
        let spacing_y = height / 4;

        let mut index = 0;
        let mut place = |organismo: fn(i32, i32, &mut dyn FnMut(i32, i32))| {
            let x = (index % 4) * spacing_x + 2;
            let y = (index / 4) * spacing_y + 2;
            organismo(x, y, &mut set);
            index += 1;
        };

        place(pulsar);
        place(lwss);
        place(mwss);
        place(pulsar);
        place(beacon);
        place(pentadecathlon);
        place(block);
        place(loaf);
        place(lwss);
        place(mwss);
        place(boat);
        place(tub);
        place(pulsar);
        place(glider);
        place(beacon);
        place(pulsar);
    }
}
