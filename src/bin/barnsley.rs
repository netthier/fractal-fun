use macroquad::prelude::*;
use macroquad::rand::gen_range;

const BATCH_SIZE: usize = 2048;

#[macroquad::main("Barnsley Fern")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    let transform = Vec2::new(width / 2.0, height - 1.0);
    let scale = if height > width*2.0 {
        Vec2::new(width / 5.0, -width / 5.0)
    } else {
        Vec2::new(height / 12.0, -height / 12.0)
    };

    let mut buffer = Image {
        width: width as u16,
        height: height as u16,
        bytes: vec![0; 4 * width as usize * height as usize],
    };

    let target = render_target(width as u32, height as u32);

    let f1 = (Mat2::from_cols_array(&[0.0, 0.0, 0.0, 0.16]), Vec2::new(0.0, 0.0));
    let f2 = (Mat2::from_cols_array(&[0.85, -0.04, 0.04, 0.85]), Vec2::new(0.0, 1.6));
    let f3 = (Mat2::from_cols_array(&[0.2, 0.23, -0.26, 0.22]), Vec2::new(0.0, 1.6));
    let f4 = (Mat2::from_cols_array(&[-0.15, 0.26, 0.28, 0.24]), Vec2::new(0.0, 0.44));

    let mut coords = Vec2::new(0.0, 0.0);
    let texture = Texture2D::from_image(&buffer);

    loop {
        buffer.bytes = vec![0; 4 * width as usize * height as usize];
        for _ in 0..BATCH_SIZE {
            coords = match gen_range(0, 99) {
                0..=0 => f1.0 * coords + f1.1,
                1..=85 => f2.0 * coords + f2.1,
                86..=92 => f3.0 * coords + f3.1,
                93..=99 => f4.0 * coords + f4.1,
                _ => unreachable!(),
            };

            let real = coords * scale + transform;
            buffer.set_pixel(real.x.round() as u32, real.y.round() as u32, Color::from_rgba(0, 128, 0, 64));
        }
        set_camera(&Camera2D {
            render_target: Some(target),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height))
        });
        texture.update(&buffer);
        draw_texture(texture, 0.0, 0.0, WHITE);
        set_default_camera();
        clear_background(WHITE);
        draw_texture_ex(target.texture, 0.0, 0.0, WHITE, DrawTextureParams { flip_y: true, ..DrawTextureParams::default() });
        next_frame().await;
    }
}
