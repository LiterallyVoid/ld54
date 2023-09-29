use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let font = load_ttf_font("data/font.ttf")
        .await
		.ok();
	let font = font.as_ref();

	loop {
		let gl = unsafe { get_internal_gl().quad_gl };
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text_ex("HELLO", 20.0, 20.0, TextParams {
			font_size: 20,
			font,
			..Default::default()
		});

        next_frame().await
    }
}
