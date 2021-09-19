use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod color;
mod light;
mod vector;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const VIEWPORT_DISTANCE: f64 = 1.0;
const VIEWPORT_HEIGHT: u32 = 1;
const VIEWPORT_WIDTH: u32 = 1;

type Canvas = [[color::Color; WIDTH as usize]; HEIGHT as usize];

const BACKGROUND_COLOR: color::Color = color::BACKGROUND_GREEN;

struct Sphere {
    c: vector::Vec3,
    r: f32,
    color: color::Color,
}

struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<light::Light>,
}

/**  "Because the viewport is measured in world units and the canvas
 * is measured in pixels, going from canvas coordinates to space
 * coordinates is just a change of scale"
 */
fn canvas_to_viewport(p: &vector::Vec2) -> vector::Vec3 {
    return vector::Vec3 {
        x: p.x * ((VIEWPORT_WIDTH as f64) / (WIDTH as f64)),
        y: p.y * ((VIEWPORT_HEIGHT as f64) / (HEIGHT as f64)),
        z: VIEWPORT_DISTANCE,
    };
}

/**
 * Returns the scalar value t for the ray equation where the given ray intersects
 * with the given sphere
 */
fn intersect_ray_sphere(from: &vector::Vec3, to: &vector::Vec3, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.r;
    let co = vector::sub3(from, &sphere.c);

    let a = vector::dot3(to, to) as f32;
    let b = 2.0 * vector::dot3(&co, to) as f32;
    let c = (vector::dot3(&co, &co) as f32) - r * r;

    // quadratic
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (f64::MAX, f64::MAX);
    }
    let t1 = (-(b as f64) + (discriminant as f64).sqrt()) / (2.0 * a) as f64;
    let t2 = (-(b as f64) - (discriminant as f64).sqrt()) / (2.0 * a) as f64;

    return (t1, t2);
}

/**
 * Given a 2d array of colors, draw them into the frame buffer
 */
fn draw_canvas(canvas: Canvas, frame: &mut [u8]) {
    for (y, row) in canvas.iter().enumerate() {
        for (x, color) in row.iter().enumerate() {
            let pixel_start = y * (WIDTH as usize) * 4 + x * 4;
            frame[pixel_start] = color.r;
            frame[pixel_start + 1] = color.g;
            frame[pixel_start + 2] = color.b;
            frame[pixel_start + 3] = color.a;
        }
    }
}

fn trace_ray(
    from: &vector::Vec3,
    to: &vector::Vec3,
    min: f64,
    max: f64,
    scene: &Scene,
) -> color::Color {
    let mut closest_t: f64 = f64::MAX;
    let mut closest_sphere: Option<&Sphere> = None;
    for sphere in scene.spheres.iter() {
        let (t1, t2) = intersect_ray_sphere(from, to, sphere);
        if (min..max).contains(&t1) && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if (min..max).contains(&t2) && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }
    match closest_sphere {
        Some(sphere) => {
            let color = sphere.color;
            let ray_point = vector::add3(from, &vector::scalar_product3(to, closest_t));
            let normal_ = vector::sub3(&ray_point, &sphere.c);
            let normal = vector::scalar_div3(&normal_, vector::magnitude3(&normal_));
            return color::multiply(
                &color,
                light::compute_lighting(&ray_point, &normal, &scene.lights),
            );
        }
        None => {
            return BACKGROUND_COLOR;
        }
    }
}

fn draw_pixel(canvas: &mut Canvas, p: &vector::Vec2, c: color::Color) {
    let canvas_x = p.x + (WIDTH as f64) / 2.0;
    let canvas_y = -1.0 * (p.y - (HEIGHT as f64) / 2.0) - 1.0;
    canvas[canvas_y as usize][canvas_x as usize] = c;
}

fn raytrace_to_canvas(canvas: &mut Canvas, o: &vector::Vec3, scene: &Scene) {
    for x in -((WIDTH as i32) / 2)..((WIDTH as i32) / 2) {
        for y in -((HEIGHT as i32) / 2)..((HEIGHT as i32) / 2) {
            let canvas_point = vector::Vec2 {
                x: x as f64,
                y: y as f64,
            };
            let d = canvas_to_viewport(&canvas_point);
            let color = trace_ray(o, &d, 1.0, f64::MAX, scene);
            draw_pixel(canvas, &canvas_point, color);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

    let mut canvas: Canvas = [[color::Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    }; WIDTH as usize]; HEIGHT as usize];

    let origin: vector::Vec3 = vector::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut scene: Scene = Scene {
        spheres: Vec::new(),
        lights: Vec::new(),
    };
    scene.spheres.push(Sphere {
        c: vector::Vec3 {
            x: 0.0,
            y: -0.5,
            z: 6.0,
        },
        r: 1.0,
        color: color::PINK,
    });
    scene.spheres.push(Sphere {
        c: vector::Vec3 {
            x: 2.0,
            y: 0.0,
            z: 4.0,
        },
        r: 0.3,
        color: color::GREEN,
    });
    scene.spheres.push(Sphere {
        c: vector::Vec3 {
            x: -2.0,
            y: 0.0,
            z: 4.0,
        },
        r: 1.2,
        color: color::YELLOW,
    });
    scene.spheres.push(Sphere {
        c: vector::Vec3 {
            x: 1.0,
            y: -1.0,
            z: 3.0,
        },
        r: 0.3,
        color: color::GREEN,
    });
    scene.lights.push(light::Light::Ambient { intensity: 0.3 });
    scene.lights.push(light::Light::Point {
        intensity: 0.7,
        position: vector::Vec3 {
            x: -2.0,
            y: 1.0,
            z: 0.0,
        },
    });
    scene.lights.push(light::Light::Directional {
        intensity: 0.9,
        direction: vector::Vec3 {
            x: 1.0,
            y: 4.0,
            z: 4.0,
        },
    });

    let frame = pixels.get_frame();
    raytrace_to_canvas(&mut canvas, &origin, &scene);
    draw_canvas(canvas, frame);
    pixels.render().unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
