use std::cmp;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn multiply(c: &Color, x: f64) -> Color {
    return Color {
        r: cmp::min(((c.r as f64) * x).round() as i32, 255) as u8,
        g: cmp::min(((c.g as f64) * x).round() as i32, 255) as u8,
        b: cmp::min(((c.b as f64) * x).round() as i32, 255) as u8,
        a: cmp::min(((c.a as f64) * x).round() as i32, 255) as u8,
    };
}

pub const BACKGROUND_GREEN: Color = Color {
    r: 0,
    g: 56,
    b: 68,
    a: 255,
};

pub const GREEN: Color = Color {
    r: 0,
    g: 108,
    b: 103,
    a: 255,
};

pub const PINK: Color = Color {
    r: 241,
    g: 148,
    b: 180,
    a: 255,
};

pub const YELLOW: Color = Color {
    r: 255,
    g: 177,
    b: 0,
    a: 255,
};
