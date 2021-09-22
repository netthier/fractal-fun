use macroquad::prelude::*;

const ITERATIONS: usize = 8;

#[macroquad::main("Zig Zag")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    let mut points = vec![Vec2::new(width, height), Vec2::new(0.0, height)];

    for _ in 0..ITERATIONS {
        points = points.windows(2).flat_map(|ab| fractalize(ab[0], ab[1])).collect();
    }

    dbg!(points.len(), std::mem::size_of::<Vec2>() * points.len() / 1000);
    
    for ab in points.windows(2) {
        let a = ab[0];
        let b = ab[1];
        draw_line(a.x, a.y, b.x, b.y, 1.0, RED);
    }
    
    next_frame().await;
    std::thread::park();
}

fn fractalize(a: Vec2, b: Vec2) -> Vec<Vec2> {
    let lerp = a.lerp(b, 1.0 / 3.0);

    if a.x != b.x {
        let delta = lerp.x - a.x;
        vec![
            a,
            lerp,
            lerp + Vec2::new(0.0, delta),
            lerp + Vec2::new(delta, delta),
            lerp + Vec2::new(delta, 0.0),
            b
        ]
    } else {
        let delta = lerp.y - a.y;
        vec![
            a,
            lerp,
            lerp + Vec2::new(-delta, 0.0),
            lerp + Vec2::new(-delta, delta),
            lerp + Vec2::new(0.0, delta),
            b
        ]
    }
}
