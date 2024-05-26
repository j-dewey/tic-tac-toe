const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 0];

fn between(num: f32, lower: f32, upper: f32) -> bool {
    lower <= num && upper >= num
}

fn any_in_range(
    lx: i32,
    ux: i32,
    ly: i32,
    uy: i32,
    check: f32,
    filter: fn(i32, i32) -> f32,
) -> bool {
    for x in lx..ux {
        for y in ly..uy {
            if (filter)(x, y) == check {
                return true;
            }
        }
    }
    return false;
}

pub fn draw_x(width: u32, height: u32, line_width: f32) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    let up_slope = width as f32 / height as f32;
    for y in 0..height {
        let ux = up_slope * y as f32;
        // just flipping across center
        let dx = (width as f32) / 2.0 - ux + (width as f32) / 2.0;
        for x in 0..width {
            let x = x as f32;
            let color = if between(x, ux - line_width / 2.0, ux + line_width / 2.0)
                || between(x, dx - line_width / 2.0, dx + line_width / 2.0)
            {
                BLACK
            } else {
                WHITE
            };
            bytes.extend(color);
        }
    }
    bytes
}

pub fn draw_o(width: u32, height: u32, line_width: f32) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..=height {
        let sy = (y as f32 / height as f32) * 2.0 - 1.0;
        // trig gives value between 0 and 1, which is scaled back to
        // physical position then scaled
        let dx = (sy.asin().cos() * width as f32 / 2.0) + (width as f32 / 2.0);
        let ux = (width as f32) / 2.0 - dx + (width as f32) / 2.0; // just flipping across center
        if y == height {
            println!("physical y: {}", y);
            println!("screen y: {}", sy);
            println!("arcsin: {}", sy.asin());
            println!("cosin: {}", sy.asin().cos());
        }
        for x in 0..width {
            let x = x as f32;
            let color = if between(x, ux - line_width / 2.0, ux + line_width / 2.0)
                || between(x, dx - line_width / 2.0, dx + line_width / 2.0)
            {
                BLACK
            } else {
                WHITE
            };
            bytes.extend(color);
        }
    }
    bytes
}

pub fn draw_grid(width: u32, height: u32, line_width: f32) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    let box_height = height / 3;
    let box_width = width / 3;
    // horizontal / vertical line 1 / 2
    let hl1 = height as f32 / 3.0;
    let hl2 = hl1 * 2.0;
    let vl1 = width as f32 / 3.0;
    let vl2 = vl1 * 2.0;
    let hlw = line_width / 2.0;
    for y in 0..height {
        let y_check =
            between(y as f32, hl1 - hlw, hl1 + hlw) || between(y as f32, hl2 - hlw, hl2 + hlw);
        for x in 0..width {
            let x_check =
                between(x as f32, vl1 - hlw, vl1 + hlw) || between(x as f32, vl2 - hlw, vl2 + hlw);
            let color = if x_check || y_check { BLACK } else { WHITE };
            bytes.extend(color);
        }
    }
    bytes
}

pub fn draw_weird_shapes(width: u32, line_width: f32) -> Vec<u8> {
    // change the omparison to radius to see cool things
    let mut bytes: Vec<u8> = Vec::with_capacity((width * width * 4) as usize);
    let radius = width as f32 / 2.0;
    let center = width as f32 / 2.0;
    for y in 0..=width {
        let y = y as f32;
        let dist_y = (y - center) * (y - center);
        for x in 0..=width {
            let x = x as f32;
            let dist_x = (x - center) * (x - center);
            let color = if (dist_x + dist_y).sqrt() == radius {
                BLACK
            } else {
                WHITE
            };
            bytes.extend(color);
        }
    }
    bytes
}
