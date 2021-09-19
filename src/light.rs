use crate::vector;

pub enum Light {
    Ambient {
        intensity: f64,
    },
    Point {
        intensity: f64,
        position: vector::Vec3,
    },
    Directional {
        intensity: f64,
        direction: vector::Vec3,
    },
}

fn directional_intensity(dir: &vector::Vec3, normal: &vector::Vec3, i: f64) -> f64 {
    let normal_dot_dir = vector::dot3(normal, dir);
    if normal_dot_dir > 0.0 {
        return i * normal_dot_dir / (vector::magnitude3(normal) * vector::magnitude3(dir));
    }
    return 0.0;
}

/**
 * For a given point and normal and a list of lights, compute the intensity at that point.
 */
pub fn compute_lighting(p: &vector::Vec3, normal: &vector::Vec3, lights: &Vec<Light>) -> f64 {
    let mut i = 0.0;
    for light in lights {
        match light {
            Light::Ambient { intensity } => i += intensity,
            Light::Point {
                intensity,
                position,
            } => i += directional_intensity(&vector::sub3(&position, p), normal, *intensity),
            Light::Directional {
                intensity,
                direction,
            } => i += directional_intensity(&direction, normal, *intensity),
        }
    }
    return i;
}
