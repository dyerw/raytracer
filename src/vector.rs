pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn dot2(v1: &Vec2, v2: &Vec2) -> f64 {
    return v1.x * v2.x + v1.y * v2.y;
}

pub fn dot3(v1: &Vec3, v2: &Vec3) -> f64 {
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
}

pub fn sub3(v1: &Vec3, v2: &Vec3) -> Vec3 {
    return Vec3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    };
}
