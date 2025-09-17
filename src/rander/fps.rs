// rander/fps.rs

const FONT_WIDTH: usize = 3;
const FONT_HEIGHT: usize = 5;

// Jede Zahl ist 3x5 Pixel, 1 = Pixel an, 0 = Pixel aus
const DIGIT_FONT: [[u8; FONT_HEIGHT]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111], // 0
    [0b010, 0b110, 0b010, 0b010, 0b111], // 1
    [0b111, 0b001, 0b111, 0b100, 0b111], // 2
    [0b111, 0b001, 0b111, 0b001, 0b111], // 3
    [0b101, 0b101, 0b111, 0b001, 0b001], // 4
    [0b111, 0b100, 0b111, 0b001, 0b111], // 5
    [0b111, 0b100, 0b111, 0b101, 0b111], // 6
    [0b111, 0b001, 0b001, 0b010, 0b010], // 7
    [0b111, 0b101, 0b111, 0b101, 0b111], // 8
    [0b111, 0b101, 0b111, 0b001, 0b111], // 9
];

const CHAR_FONT: [(char, [u8; FONT_HEIGHT]); 5] = [
    ('F', [0b111, 0b100, 0b111, 0b100, 0b100]),
    ('P', [0b111, 0b101, 0b111, 0b100, 0b100]),
    ('S', [0b111, 0b100, 0b111, 0b001, 0b111]),
    (':', [0b000, 0b010, 0b000, 0b010, 0b000]),
    (' ', [0b000, 0b000, 0b000, 0b000, 0b000]),
];

fn draw_digit(x: usize, y: usize, HEIGHT: usize, WIDTH: usize, digit: u8, color: u32, buffer: &mut [u32]) {
    if digit > 9 {
        return;
    }

    let glyph = DIGIT_FONT[digit as usize];

    for (row_idx, row) in glyph.iter().enumerate() {
        for col in 0..FONT_WIDTH {
            if (row >> (FONT_WIDTH - 1 - col)) & 1 == 1 {
                let px = x + col;
                let py = y + row_idx;
                if px < WIDTH && py < HEIGHT {
                    buffer[py * WIDTH + px] = color;
                }
            }
        }
    }
}

fn draw_char(x: usize, y: usize, HEIGHT: usize, WIDTH: usize, ch: char, color: u32, buffer: &mut [u32]) {
    if let Some((_, glyph)) = CHAR_FONT.iter().find(|(c, _)| *c == ch) {
        for (row_idx, row) in glyph.iter().enumerate() {
            for col in 0..FONT_WIDTH {
                if (row >> (FONT_WIDTH - 1 - col)) & 1 == 1 {
                    let px = x + col;
                    let py = y + row_idx;
                    if px < WIDTH && py < HEIGHT {
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }
    }
}

pub fn draw_Fps(mut x: usize, y: usize, HEIGHT: usize, WIDTH: usize, text: &str, color: u32, buffer: &mut [u32]) {
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            draw_digit(x, y, HEIGHT, WIDTH, ch as u8 - b'0', color, buffer);
        } else {
            draw_char(x, y, HEIGHT, WIDTH, ch, color, buffer);
        }
        x += FONT_WIDTH + 1; // Abstand zwischen Zeichen
    }
}
