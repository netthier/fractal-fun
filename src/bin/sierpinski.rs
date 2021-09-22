use macroquad::prelude::*;

#[derive(Copy, Clone)]
struct Tri(Vec2, Vec2, Vec2);

const ITERATIONS: usize = 8;

#[macroquad::main("Sierpinski")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    let init = Tri(Vec2::new(0.0, height), Vec2::new(width, height), Vec2::new(width / 2.0, 0.0));
    
    let mut tris = vec![init];

    for _ in 0..ITERATIONS {
        tris = tris.iter().flat_map(|e| fractalize_tri(*e)).collect();
    }

    dbg!(tris.len(), std::mem::size_of::<Tri>() * tris.len() / 1000);
    
    for tri in tris.iter() {
        draw_tri(*tri);
    }
    
    next_frame().await;
    std::thread::park();
}

fn draw_tri(tri: Tri) {
    draw_line(tri.0.x, tri.0.y, tri.1.x, tri.1.y, 1.0, RED);
    draw_line(tri.1.x, tri.1.y, tri.2.x, tri.2.y, 1.0, RED);
    draw_line(tri.2.x, tri.2.y, tri.0.x, tri.0.y, 1.0, RED);
}

fn fractalize_tri(tri: Tri) -> Vec<Tri> {
    vec![
        Tri(tri.0, tri.0.lerp(tri.1, 0.5), tri.0.lerp(tri.2, 0.5)),
        Tri(tri.1, tri.1.lerp(tri.0, 0.5), tri.1.lerp(tri.2, 0.5)),
        Tri(tri.2, tri.2.lerp(tri.0, 0.5), tri.2.lerp(tri.1, 0.5))
    ]
}
